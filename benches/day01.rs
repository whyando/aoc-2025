use aoc_2025::day01;
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use std::fs;

fn bench_day01_solve(c: &mut Criterion) {
    let input = fs::read_to_string("inputs/01.txt").unwrap();

    c.bench_function("day01_solve", |b| {
        b.iter(|| {
            let _ = day01::solve(black_box(&input));
        });
    });
}

fn bench_day01_read_to_string(c: &mut Criterion) {
    c.bench_function("day01_read_to_string", |b| {
        b.iter(|| {
            let _ = fs::read_to_string(black_box("inputs/01.txt")).unwrap();
        });
    });
}

criterion_group!(benches, bench_day01_solve, bench_day01_read_to_string);
criterion_main!(benches);
