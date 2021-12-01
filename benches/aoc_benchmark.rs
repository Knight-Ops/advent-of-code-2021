use criterion::{black_box, criterion_group, criterion_main, Criterion};
use advent_of_code_2021::*;

pub fn day1_benchmark(c: &mut Criterion) {
    let input = day1::input_generator(&read_input_file("input/2021/day1.txt"));
    c.bench_function("day 1 part 1", |b| b.iter(|| day1::part1(&input)));
    c.bench_function("day 1 part 2", |b| b.iter(|| day1::part2(&input)));
}

criterion_group!(benches, day1_benchmark);
criterion_main!(benches);