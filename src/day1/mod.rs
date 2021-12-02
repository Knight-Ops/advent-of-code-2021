pub fn input_generator(input: &str) -> Vec<u32> {
    input
        .split("\n")
        .into_iter()
        .map(|num| {
            num.trim()
                .parse::<u32>()
                .expect("Error parsing &str into u32")
        })
        .collect()
}

pub fn part1(input: &[u32]) -> usize {
    input
        .windows(2)
        .fold(0, |acc, x| if x[0] < x[1] { acc + 1 } else { acc })
}

pub fn part2(input: &[u32]) -> usize {
    let mut p = input
        .windows(3)
        .map(|x| x.iter().fold(0, |acc, x| acc + x))
        .peekable();

    let mut count = 0;
    while let Some(x) = p.next() {
        if let Some(&p) = p.peek() {
            if x < p {
                count += 1;
            }
        }
    }
    count
}

pub fn part2_lookback(input: &[u32]) -> usize {
    let sliding_window = input.windows(3).map(|x| x.iter().fold(0, |acc, x| acc + x));

    let mut count = 0;
    let mut last = None;
    for ea in sliding_window {
        if let Some(last_val) = last {
            if last_val < ea {
                count += 1;
            }
        }

        last = Some(ea);
    }
    count
}

pub fn part2_orig(input: &[u32]) -> usize {
    let sliding_window: Vec<u32> = input.windows(3).map(|x| x.iter().sum()).collect();

    sliding_window
        .iter()
        .enumerate()
        .take_while(|(idx, _)| *idx < sliding_window.len() - 1)
        .filter(|(idx, x)| **x < sliding_window[idx + 1])
        .count()
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

    test!(part1, 7);
    test!(part2, 5);
    test!(part2_lookback, 5);
    test!(part2_orig, 5);
}
