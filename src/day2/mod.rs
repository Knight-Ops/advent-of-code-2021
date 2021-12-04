#[derive(Debug, Clone)]
pub enum Direction {
    Forward(u32),
    Down(u32),
    Up(u32),
}

pub fn input_generator(input: &str) -> Vec<Direction> {
    input
        .split("\n")
        .into_iter()
        .map(|x| {
            let mut command = x.split(" ");
            let cmd_str = command
                .next()
                .expect("Error while getting directional command");
            let cmd_val = command
                .next()
                .expect("Error while getting directional value")
                .trim()
                .parse::<u32>()
                .expect("Error while parsing u32 from cmd_val");

            match cmd_str {
                "forward" => Direction::Forward(cmd_val),
                "up" => Direction::Up(cmd_val),
                "down" => Direction::Down(cmd_val),
                _ => panic!("Unexpected input while parsing direction"),
            }
        })
        .collect()
}

pub fn part1(input: &[Direction]) -> usize {
    let mut horizontal = 0;
    let mut depth = 0;

    for dir in input {
        match dir {
            Direction::Up(val) => depth -= val,
            Direction::Down(val) => depth += val,
            Direction::Forward(val) => horizontal += val,
        }
    }

    (horizontal * depth) as usize
}

pub fn part2(input: &[Direction]) -> usize {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for dir in input {
        match dir {
            Direction::Up(val) => aim -= val,
            Direction::Down(val) => aim += val,
            Direction::Forward(val) => {
                horizontal += val;
                depth += aim * val
            }
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
                let i = read_input_file(&format!(
                    "input/2021/{}_test.txt",
                    name[name.len() - 2].trim()
                ));

                let input = super::input_generator(&i);
                assert_eq!(super::$func(&input), $val);
            }
        };
    }

    test!(part1, 150);
    test!(part2, 900);
}
