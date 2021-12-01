pub fn input_generator(input: &str) -> Vec<u32> {
    input
        .split("\n")
        .into_iter()
        .map(|num| num.trim().parse::<u32>().expect("Error parsing &str into u32"))
        .collect()
}

pub fn part1(input: &[u32]) -> usize {

}

pub fn part2(input: &[u32]) -> usize {

}

#[cfg(test)]
mod tests {
    use crate::read_input_file;
    use super::*;

    #[test]
    fn part1_test() {

    }

    #[test]
    fn part2_test() {

    }
}
