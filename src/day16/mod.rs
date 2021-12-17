use std::cell::RefCell;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct BinaryArray {
    inner: RefCell<VecDeque<u8>>,
}

impl BinaryArray {
    pub fn get_bits(&self, count: usize) -> usize {
        let mut value = 0;

        self.inner
            .borrow_mut()
            .drain(0..count)
            .for_each(|bit| value = (value << 1) | bit as usize);

        value
    }

    pub fn sub_array(&self, count: usize) -> Self {
        let tmp_vecdeque = self.inner.borrow_mut().drain(0..count).collect();

        BinaryArray {
            inner: RefCell::new(tmp_vecdeque),
        }
    }
}

impl From<&str> for BinaryArray {
    fn from(input: &str) -> Self {
        let mut binary_array = VecDeque::with_capacity(input.len() * 4);

        input.chars().for_each(|c| match c {
            '0' => binary_array.extend([0, 0, 0, 0].iter()),
            '1' => binary_array.extend([0, 0, 0, 1].iter()),
            '2' => binary_array.extend([0, 0, 1, 0].iter()),
            '3' => binary_array.extend([0, 0, 1, 1].iter()),
            '4' => binary_array.extend([0, 1, 0, 0].iter()),
            '5' => binary_array.extend([0, 1, 0, 1].iter()),
            '6' => binary_array.extend([0, 1, 1, 0].iter()),
            '7' => binary_array.extend([0, 1, 1, 1].iter()),
            '8' => binary_array.extend([1, 0, 0, 0].iter()),
            '9' => binary_array.extend([1, 0, 0, 1].iter()),
            'A' => binary_array.extend([1, 0, 1, 0].iter()),
            'B' => binary_array.extend([1, 0, 1, 1].iter()),
            'C' => binary_array.extend([1, 1, 0, 0].iter()),
            'D' => binary_array.extend([1, 1, 0, 1].iter()),
            'E' => binary_array.extend([1, 1, 1, 0].iter()),
            'F' => binary_array.extend([1, 1, 1, 1].iter()),
            _ => panic!("Invalid input character in hex packet"),
        });

        BinaryArray {
            inner: RefCell::new(binary_array),
        }
    }
}

#[derive(Debug, Clone)]
pub enum OperationType {
    Sum,
    Product,
    Min,
    Max,
    GreaterThan,
    LessThan,
    EqualTo,
}

#[derive(Debug, Clone)]
pub enum LengthTypeId {
    Bits(usize),
    Packets(usize),
}

#[derive(Debug, Clone)]
pub enum PacketType {
    Literal(usize),
    Operator {
        operation_type: OperationType,
        length_type_id: LengthTypeId,
        sub_packets: Vec<Packet>,
    },
}

impl PacketType {
    fn from(input: &BinaryArray, type_id: u8) -> PacketType {
        match type_id {
            4 => {
                let mut literal = 0;

                let mut last_nibble = false;
                while !last_nibble {
                    let nibble = input.get_bits(5);

                    if nibble & 0b10000 == 0 {
                        last_nibble = true;
                    }

                    literal = (literal << 4) | (nibble & 0b1111);
                }

                PacketType::Literal(literal)
            }
            _ => {
                let operation_type = match type_id {
                    0 => OperationType::Sum,
                    1 => OperationType::Product,
                    2 => OperationType::Min,
                    3 => OperationType::Max,
                    5 => OperationType::GreaterThan,
                    6 => OperationType::LessThan,
                    7 => OperationType::EqualTo,
                    _ => unimplemented!("This operation type has not been implemented"),
                };

                let length_type_id = match input.get_bits(1) {
                    0 => LengthTypeId::Bits(input.get_bits(15)),
                    1 => LengthTypeId::Packets(input.get_bits(11)),
                    _ => unimplemented!("This length type id has not been implemented"),
                };

                let mut sub_packets = Vec::new();
                match length_type_id {
                    LengthTypeId::Bits(num_bits) => {
                        let sub_packet_array = input.sub_array(num_bits);
                        while sub_packet_array.inner.borrow().len() > 7 {
                            sub_packets.push(Packet::from(&sub_packet_array));
                        }
                    }
                    LengthTypeId::Packets(pkt_count) => {
                        for _ in 0..pkt_count {
                            sub_packets.push(Packet::from(input));
                        }
                    }
                }

                PacketType::Operator {
                    operation_type,
                    length_type_id,
                    sub_packets,
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Packet {
    version: u8,
    type_id: u8,
    packet_type: PacketType,
}

impl Packet {
    fn sum_versions(&self) -> usize {
        match &self.packet_type {
            PacketType::Literal(_) => self.version as usize,
            PacketType::Operator {
                operation_type: _,
                length_type_id: _,
                sub_packets,
            } => {
                sub_packets
                    .iter()
                    .map(|pkt| pkt.sum_versions())
                    .sum::<usize>()
                    + self.version as usize
            }
        }
    }

    fn evaluate(&self) -> usize {
        match &self.packet_type {
            PacketType::Literal(val) => *val,
            PacketType::Operator {
                operation_type,
                length_type_id: _,
                sub_packets,
            } => match operation_type {
                OperationType::Sum => sub_packets.iter().map(|pkt| pkt.evaluate()).sum(),
                OperationType::Product => sub_packets
                    .iter()
                    .map(|pkt| pkt.evaluate())
                    .fold(1, |acc, x| acc * x),
                OperationType::Min => sub_packets
                    .iter()
                    .map(|pkt| pkt.evaluate())
                    .min()
                    .expect("Could not find any min value"),
                OperationType::Max => sub_packets
                    .iter()
                    .map(|pkt| pkt.evaluate())
                    .max()
                    .expect("Could not find any max value"),
                OperationType::GreaterThan => {
                    if sub_packets.len() != 2 {
                        unreachable!("This operation should always have exactly 2 sub_packets");
                    }

                    if sub_packets[0].evaluate() > sub_packets[1].evaluate() {
                        1
                    } else {
                        0
                    }
                }
                OperationType::LessThan => {
                    if sub_packets.len() != 2 {
                        unreachable!("This operation should always have exactly 2 sub_packets");
                    }

                    if sub_packets[0].evaluate() < sub_packets[1].evaluate() {
                        1
                    } else {
                        0
                    }
                }
                OperationType::EqualTo => {
                    if sub_packets.len() != 2 {
                        unreachable!("This operation should always have exactly 2 sub_packets");
                    }

                    if sub_packets[0].evaluate() == sub_packets[1].evaluate() {
                        1
                    } else {
                        0
                    }
                }
            },
        }
    }
}

impl From<&BinaryArray> for Packet {
    fn from(input: &BinaryArray) -> Self {
        let version = input.get_bits(3) as u8;
        let type_id = input.get_bits(3) as u8;

        let packet_type = PacketType::from(input, type_id);

        Packet {
            version,
            type_id,
            packet_type,
        }
    }
}

pub fn input_generator(input: &str) -> BinaryArray {
    BinaryArray::from(input)
}

pub fn part1(input: &BinaryArray) -> usize {
    let packets = Packet::from(&input.clone());

    packets.sum_versions()
}

pub fn part2(input: &BinaryArray) -> usize {
    let packets = Packet::from(&input.clone());

    packets.evaluate()
}

#[cfg(test)]
mod tests {
    use crate::read_input_file;
    macro_rules! test {
        ($func:ident, $val:expr) => {
            #[test]
            fn $func() {
                let name = module_path!().split("::").collect::<Vec<&str>>();
                let i = read_input_file(&format!(
                    "input/2021/{}_test.txt",
                    name[name.len() - 2].trim()
                ));

                let input = super::input_generator(&i);
                assert_eq!(super::$func(&input), $val);
            }
        };
    }

    #[test]
    fn part1_test1() {
        let input = super::input_generator("8A004A801A8002F478");
        assert_eq!(super::part1(&input), 16);
    }

    #[test]
    fn part1_test2() {
        let input = super::input_generator("620080001611562C8802118E34");
        assert_eq!(super::part1(&input), 12);
    }

    #[test]
    fn part1_test3() {
        let input = super::input_generator("C0015000016115A2E0802F182340");
        assert_eq!(super::part1(&input), 23);
    }

    #[test]
    fn part1_test4() {
        let input = super::input_generator("A0016C880162017C3686B18A3D4780");
        assert_eq!(super::part1(&input), 31);
    }

    #[test]
    fn part2_test1() {
        let input = super::input_generator("C200B40A82");
        assert_eq!(super::part2(&input), 3);
    }

    #[test]
    fn part2_test2() {
        let input = super::input_generator("04005AC33890");
        assert_eq!(super::part2(&input), 54);
    }

    #[test]
    fn part2_test3() {
        let input = super::input_generator("880086C3E88112");
        assert_eq!(super::part2(&input), 7);
    }

    #[test]
    fn part2_test4() {
        let input = super::input_generator("CE00C43D881120");
        assert_eq!(super::part2(&input), 9);
    }

    #[test]
    fn part2_test5() {
        let input = super::input_generator("D8005AC2A8F0");
        assert_eq!(super::part2(&input), 1);
    }

    #[test]
    fn part2_test6() {
        let input = super::input_generator("F600BC2D8F");
        assert_eq!(super::part2(&input), 0);
    }

    #[test]
    fn part2_test7() {
        let input = super::input_generator("9C005AC2F8F0");
        assert_eq!(super::part2(&input), 0);
    }

    #[test]
    fn part2_test8() {
        let input = super::input_generator("9C0141080250320F1802104A08");
        assert_eq!(super::part2(&input), 1);
    }
}
