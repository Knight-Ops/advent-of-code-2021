use advent_of_code_2021::*;

macro_rules! run {
    ($lib:ident) => {
        {
            let raw_input = read_input_file(&format!("input/2021/{}.txt", stringify!($lib)));

            let formatted_input = $lib::input_generator(&raw_input);

            println!("================= {} =================", stringify!($lib));

            let part1 = $lib::part1(&formatted_input);
            println!("Solution for {} Part 1 : {}", stringify!($lib), part1);

            let part2 = $lib::part2(&formatted_input);
            println!("Solution for {} Part 2: {}", stringify!($lib), part2);
        }
    };
    ($lib:ident, $($func:ident), +) => {
        {
            let raw_input = read_input_file(&format!("input/2021/{}.txt", stringify!($lib)));

            let formatted_input = $lib::input_generator(&raw_input);

            println!("================= {} =================", stringify!($lib));

            let part1 = $lib::part1(&formatted_input);
            println!("Solution for {} Part 1 : {}", stringify!($lib), part1);

            let part2 = $lib::part2(&formatted_input);
            println!("Solution for {} Part 2: {}", stringify!($lib), part2);

            $(println!("Solution for {} {} : {}", stringify!($lib), stringify!($func), $lib::$func(&formatted_input));)*
        }
    }
}

macro_rules! run_mut {
    ($lib:ident) => {
        {
            let raw_input = read_input_file(&format!("input/2021/{}.txt", stringify!($lib)));

            let mut formatted_input = $lib::input_generator(&raw_input);

            println!("================= {} =================", stringify!($lib));

            let part1 = $lib::part1(&mut formatted_input);
            println!("Solution for {} Part 1 : {}", stringify!($lib), part1);

            let part2 = $lib::part2(&mut formatted_input);
            println!("Solution for {} Part 2: {}", stringify!($lib), part2);
        }
    };
    ($lib:ident, $($func:ident), +) => {
        {
            let raw_input = read_input_file(&format!("input/2021/{}.txt", stringify!($lib)));

            let mut formatted_input = $lib::input_generator(&raw_input);

            println!("================= {} =================", stringify!($lib));

            let part1 = $lib::part1(&mut formatted_input);
            println!("Solution for {} Part 1 : {}", stringify!($lib), part1);

            let part2 = $lib::part2(&mut formatted_input);
            println!("Solution for {} Part 2: {}", stringify!($lib), part2);

            $(println!("Solution for {} {} : {}", stringify!($lib), stringify!($func), $lib::$func(&formatted_input));)*
        }
    }
}

fn main() {
    run!(day1, part2_lookback, part2_orig);
    run!(day2);
    run!(day3);
    run_mut!(day4);
    run!(day5);
    run!(day6);
    run!(day7, part1_sorted, part2_naive);
    run!(day8, part2_alternate);
    run!(day9);
    run!(day10);
    run!(day11);
    run!(day12);
    run!(day13);
    run!(day14, part1_slow);
    // run!(day15);
    run!(day16);
    run!(day17);
}
