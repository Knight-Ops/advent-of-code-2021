use advent_of_code_2021 as aoc;

fn main() {
    day1();
    day2();
}

fn day1() {
    let raw_input = aoc::read_input_file("input/2021/day1.txt");

    let formatted_input = aoc::day1::input_generator(&raw_input);

    let part1 = aoc::day1::part1(&formatted_input);
    println!("Solution for Day 1 Part 1 : {}", part1);

    let part2 = aoc::day1::part2(&formatted_input);
    println!("Solution for Day 1 Part 2: {}", part2);
}

fn day2() {
    let raw_input = aoc::read_input_file("input/2021/day2.txt");

    let formatted_input = aoc::day2::input_generator(&raw_input);

    let part1 = aoc::day2::part1(&formatted_input);
    println!("Solution for Day 2 Part 1 : {}", part1);

    let part2 = aoc::day2::part2(&formatted_input);
    println!("Solution for Day 2 Part 2: {}", part2);
}
