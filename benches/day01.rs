use aoc_2025::day01;
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

fn bench_day01(c: &mut Criterion) {
    let input = std::fs::read_to_string("inputs/01.txt").unwrap();

    c.bench_function("day01_solve", |b| {
        b.iter(|| {
            let _ = day01::solve(black_box(&input));
        });
    });
}

criterion_group!(benches, bench_day01);
criterion_main!(benches);
