use advent_of_code_2021::*;
use criterion::{criterion_group, criterion_main, Criterion};

macro_rules! bench_please {
    ($lib:ident) => {
        pub fn $lib(c: &mut Criterion) {
            let raw_input = read_input_file(&format!("input/2021/{}.txt", stringify!($lib)));
            c.bench_function(&format!("{} input parser", stringify!($lib)), |b| b.iter(|| $lib::input_generator(&raw_input)));
            let input = $lib::input_generator(&raw_input);
            c.bench_function(&format!("{} part 1", stringify!($lib)), |b| b.iter(|| $lib::part1(&input)));
            c.bench_function(&format!("{} part 2", stringify!($lib)), |b| b.iter(|| $lib::part2(&input)));
        }
    };
    ($lib:ident, $($func:ident),+) => {
        pub fn $lib(c: &mut Criterion) {
            let raw_input = read_input_file(&format!("input/2021/{}.txt", stringify!($lib)));
            c.bench_function(&format!("{} input parser", stringify!($lib)), |b| b.iter(|| $lib::input_generator(&raw_input)));
            let input = $lib::input_generator(&raw_input);
            c.bench_function(&format!("{} part 1", stringify!($lib)), |b| b.iter(|| $lib::part1(&input)));
            c.bench_function(&format!("{} part 2", stringify!($lib)), |b| b.iter(|| $lib::part2(&input)));
            $(c.bench_function(&format!("{} {}", stringify!($lib), stringify!($func)), |b| b.iter(|| $lib::$func(&input)));)*
        }
    }
}

macro_rules! bench_please_mut {
    ($lib:ident) => {
        pub fn $lib(c: &mut Criterion) {
            let raw_input = read_input_file(&format!("input/2021/{}.txt", stringify!($lib)));
            c.bench_function(&format!("{} input parser", stringify!($lib)), |b| b.iter(|| $lib::input_generator(&raw_input)));
            c.bench_function(&format!("{} part 1 + parse", stringify!($lib)), |b| b.iter(|| $lib::part1(&mut $lib::input_generator(&raw_input))));
            c.bench_function(&format!("{} part 2 + parse", stringify!($lib)), |b| b.iter(|| $lib::part2(&mut $lib::input_generator(&raw_input))));
        }
    };
    ($lib:ident, $($func:ident),+) => {
        pub fn $lib(c: &mut Criterion) {
            let raw_input = read_input_file(&format!("input/2021/{}.txt", stringify!($lib)));
            c.bench_function(&format!("{} input parser", stringify!($lib)), |b| b.iter(|| $lib::input_generator(&raw_input)));
            c.bench_function(&format!("{} part 1 + parse", stringify!($lib)), |b| b.iter(|| $lib::part1(&mut $lib::input_generator(&raw_input))));
            c.bench_function(&format!("{} part 2 + parse", stringify!($lib)), |b| b.iter(|| $lib::part2(&mut $lib::input_generator(&raw_input))));
            $(c.bench_function(&format!("{} {} + parse", stringify!($lib), stringify!($func)), |b| b.iter(|| $lib::$func(&mut $lib::input_generator(&raw_input))));)*
        }
    }
}

bench_please!(day1, part2_lookback, part2_orig);
bench_please!(day2);
bench_please!(day3);
bench_please_mut!(day4);
bench_please!(day5);
bench_please!(day6);
bench_please!(day7, part1_sorted, part2_naive);
bench_please!(day8, part2_alternate);
bench_please!(day9);
bench_please!(day10);
bench_please!(day11);
bench_please!(day12);
bench_please!(day13);

criterion_group!(all, day1, day2, day3, day4, day5, day6, day7, day8, day9, day10, day11, day12, day13);
criterion_group!(single, day13);
criterion_main!(single);
