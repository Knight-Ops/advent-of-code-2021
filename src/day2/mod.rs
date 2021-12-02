pub enum Direction {
    Forward,
    Down,
    Up,
    Empty,
}

pub fn input_generator(input: &str) -> Vec<(Direction, u32)> {
    input
        .split("\n")
        .into_iter()
        .map(|x| {
            let mut d = Direction::Empty;
            let mut v = 0;
            for (idx, val) in x.split(" ").enumerate() {
                match idx {
                    0 => match val {
                        "forward" => d = Direction::Forward,
                        "up" => d = Direction::Up,
                        "down" => d = Direction::Down,
                        _ => panic!("Unexpected input while parsing direction"),
                    },
                    1 => {
                        v = val
                            .trim()
                            .parse::<u32>()
                            .expect("Error while parsing &str into u32");
                    }
                    _ => panic!("There are more than two entries in the input line"),
                }
            }

            (d, v)
        })
        .collect()
}

pub fn part1(input: &[(Direction, u32)]) -> usize {
    let mut horizontal = 0;
    let mut depth = 0;

    for (dir, val) in input {
        match dir {
            Direction::Up => depth -= val,
            Direction::Down => depth += val,
            Direction::Forward => horizontal += val,
            Direction::Empty => panic!("Empty Direction should not exist"),
        }
    }

    (horizontal * depth) as usize
}

pub fn part2(input: &[(Direction, u32)]) -> usize {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for (dir, val) in input {
        match dir {
            Direction::Up => aim -= val,
            Direction::Down => aim += val,
            Direction::Forward => {
                horizontal += val;
                depth += aim * val
            }
            Direction::Empty => panic!("Empty Direction should not exist"),
        }
    }

    (horizontal * depth) as usize
}

#[cfg(test)]
mod tests {
    use crate::read_input_file;
    macro_rules! test {
        ($func:ident, $val:expr) => {
            #[test]
            fn $func() {
                let name = module_path!().split("::").collect::<Vec<&str>>();
                let i = read_input_file(&format!("input/2021/{}_test.txt", name[name.len() - 2].trim()));

                let input = super::input_generator(&i);
                assert_eq!(super::$func(&input), $val);
            }
        }
    }

    test!(part1, 150);
    test!(part2, 900);
}
