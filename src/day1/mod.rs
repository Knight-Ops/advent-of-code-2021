pub fn input_generator(input: &str) -> Vec<u32> {
    input
        .split("\n")
        .into_iter()
        .map(|num| num.trim().parse::<u32>().expect("Error parsing &str into u32"))
        .collect()
}

pub fn part1(input: &[u32]) -> usize {
    input
        .into_iter()
        .enumerate()
        .skip(1)
        .filter(|(idx, val)| val > &&input[idx - 1])
        .count()
}

pub fn part2(input: &[u32]) -> usize {
    let sliding_window: Vec<u32> = input
        .iter()
        .enumerate()
        .take_while(|(idx, _)| *idx <= input.len() - 3)
        .map(|(idx, x)| x + input[idx + 1] + input[idx + 2])
        .collect();

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
    use super::*;

    #[test]
    fn part1_test() {
        let i = read_input_file("input/2021/day1_test.txt");

        let input = input_generator(&i);
        assert_eq!(part1(&input), 7);
    }

    #[test]
    fn part2_test() {
        let i = read_input_file("input/2021/day1_test.txt");

        let input = input_generator(&i);
        assert_eq!(part2(&input), 5);
    }
}
