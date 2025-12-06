use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;
use aoc_2025;

fn bench_day01(c: &mut Criterion) {
    let input = aoc_2025::file::read("inputs/01.txt").unwrap();
    c.bench_function("day01", |b| {
        b.iter(|| aoc_2025::day01::solve(black_box(&input)))
    });
}

fn bench_day02(c: &mut Criterion) {
    let input = std::fs::read_to_string("inputs/02.txt").unwrap();
    c.bench_function("day02", |b| {
        b.iter(|| aoc_2025::day02::solve(black_box(&input)))
    });
}

fn bench_day03(c: &mut Criterion) {
    let input = aoc_2025::file::read("inputs/03.txt").unwrap();
    c.bench_function("day03", |b| {
        b.iter(|| aoc_2025::day03::solve(black_box(&input)))
    });
}

fn bench_day04(c: &mut Criterion) {
    let input = aoc_2025::file::read_no_newlines("inputs/04.txt").unwrap();
    c.bench_function("day04", |b| {
        b.iter(|| aoc_2025::day04::solve(black_box(&input)))
    });
}

fn bench_day05(c: &mut Criterion) {
    let input = aoc_2025::file::read("inputs/05.txt").unwrap();
    c.bench_function("day05", |b| {
        b.iter(|| aoc_2025::day05::solve(black_box(&input)))
    });
}

fn bench_day06(c: &mut Criterion) {
    let input = aoc_2025::day06::read("inputs/06.txt").unwrap();
    c.bench_function("day06", |b| {
        b.iter(|| aoc_2025::day06::solve::<4>(black_box(&input)))
    });
}

criterion_group!(benches, bench_day01, bench_day02, bench_day03, bench_day04, bench_day05, bench_day06);
criterion_main!(benches);

