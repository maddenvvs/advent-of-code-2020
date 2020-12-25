use aoc2020::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fs;

pub fn day01(c: &mut Criterion) {
    let input = fs::read_to_string("../input/day-01.input").unwrap();
    let day = Day01 {};
    c.bench_function("Day 01 first part", |b| {
        b.iter(|| day.first_task(black_box(&input)))
    });
    c.bench_function("Day 01 second part", |b| {
        b.iter(|| day.second_task(black_box(&input)))
    });
}

pub fn day25(c: &mut Criterion) {
    let input = fs::read_to_string("../input/day-25.input").unwrap();
    let day = Day25 {};
    c.bench_function("Day 25 first part", |b| {
        b.iter(|| day.first_task(black_box(&input)))
    });
    c.bench_function("Day 25 second part", |b| {
        b.iter(|| day.second_task(black_box(&input)))
    });
}

criterion_group!(
    benches, // day01,
    day25,
);
criterion_main!(benches);
