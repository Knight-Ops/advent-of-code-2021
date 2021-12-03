pub fn input_generator(input: &str) -> Vec<&[u8]> {
    input
        .split("\n")
        .into_iter()
        .map(|num| num.trim().as_bytes())
        .collect()
}

fn find_least_common_bits(input: &[&[u8]]) -> Vec<u8> {
    let half = if input.len() % 2 == 0 {
        input.len() / 2
    } else {
        (input.len() / 2) + 1
    };
    let max_bit = input[0].len();
    let mut least_common_bit = vec![0; max_bit];

    'bit: for x in 0..max_bit {
        let mut ones = 0;
        for bits in input {
            if bits[x] == 0x31 {
                ones += 1;
                if ones >= half {
                    least_common_bit[x] = 0;
                    continue 'bit;
                }
            }
        }
        least_common_bit[x] = 1;
    }

    least_common_bit
}

fn find_most_common_bits(input: &[&[u8]]) -> Vec<u8> {
    let half = if input.len() % 2 == 0 {
        input.len() / 2
    } else {
        (input.len() / 2) + 1
    };
    let max_bit = input[0].len();
    let mut most_common_bit = vec![0; max_bit];

    'bit: for x in 0..max_bit {
        let mut ones = 0;
        for bits in input {
            if bits[x] == 0x31 {
                ones += 1;
                if ones >= half {
                    most_common_bit[x] = 1;
                    continue 'bit;
                }
            }
        }
        most_common_bit[x] = 0;
    }

    most_common_bit
}

fn vec_to_dec(input: &[u8]) -> usize {
    let mut ret_val = 0;

    for (idx, &bit) in input.iter().enumerate() {
        if bit > 1 {
            let bit = bit - 0x30;
            if bit != 0 {
                ret_val += usize::from(bit) << (input.len() - 1 - (idx));
            }
        } else {
            if bit != 0 {
                ret_val += usize::from(bit) << (input.len() - 1 - (idx));
            }
        }
    }

    ret_val
}

pub fn part1(input: &[&[u8]]) -> usize {
    let most_common_bit = find_most_common_bits(input);

    let gamma = vec_to_dec(&most_common_bit);

    let epsilon = (!gamma) & (1 << most_common_bit.len()) - 1;

    gamma * epsilon
}

pub fn part2(input: &[&[u8]]) -> usize {
    let mut oxygen_working_pool = input.to_vec();
    let mut co2_working_pool = input.to_vec();

    for bit in 0..input[0].len() {
        if oxygen_working_pool.len() != 1 {
            let most_common_bit = find_most_common_bits(&oxygen_working_pool);
            oxygen_working_pool = oxygen_working_pool.into_iter().filter(|&x| x[bit] - 0x30 == most_common_bit[bit]).collect();
        }
        if co2_working_pool.len() != 1 {
            let least_common_bit = find_least_common_bits(&co2_working_pool);
            co2_working_pool = co2_working_pool.into_iter().filter(|&x| x[bit] - 0x30 == least_common_bit[bit]).collect();
        }

        if oxygen_working_pool.len() == 1 && co2_working_pool.len() == 1 {
            break;
        }
    }
    if oxygen_working_pool.len() != 1 || co2_working_pool.len() != 1 {
        panic!("Sanity panic, we didn't reduce enough?");
    }

    let oxygen = vec_to_dec(oxygen_working_pool[0]);
    let co2_scrubber = vec_to_dec(co2_working_pool[0]);

    oxygen*co2_scrubber

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

    test!(part1, 198);
    test!(part2, 230);
}
