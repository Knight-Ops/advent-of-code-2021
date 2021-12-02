use advent_of_code_2021::*;
use criterion::{criterion_group, criterion_main, Criterion};

macro_rules! bench_please {
    ($lib:ident) => {
        pub fn $lib(c: &mut Criterion) {
            let input = $lib::input_generator(&read_input_file(&format!("input/2021/{}.txt", stringify!($lib))));
            c.bench_function(&format!("{} part 1", stringify!($lib)), |b| b.iter(|| $lib::part1(&input)));
            c.bench_function(&format!("{} part 2", stringify!($lib)), |b| b.iter(|| $lib::part2(&input)));
        }
    };
    ($lib:ident, $($func:ident),+) => {
        pub fn $lib(c: &mut Criterion) {
            let input = $lib::input_generator(&read_input_file(&format!("input/2021/{}.txt", stringify!($lib))));
            c.bench_function(&format!("{} part 1", stringify!($lib)), |b| b.iter(|| $lib::part1(&input)));
            c.bench_function(&format!("{} part 2", stringify!($lib)), |b| b.iter(|| $lib::part2(&input)));
            $(c.bench_function(&format!("{} {}", stringify!($lib), stringify!($func)), |b| b.iter(|| $lib::$func(&input)));)*
        }
    }
}

bench_please!(day1, part2_lookback, part2_orig);
bench_please!(day2);

criterion_group!(benches, day1, day2);
// criterion_group!(benches, day2);
criterion_main!(benches);
