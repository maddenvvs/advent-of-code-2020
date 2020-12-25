use aoc2020::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fs;

pub fn day01(c: &mut Criterion) {
    let input = fs::read_to_string("../input/day-01.input").unwrap();
    let day = Day01 {};
    c.bench_function("Day 01 first task", |b| {
        b.iter(|| day.first_task(black_box(&input)))
    });
    c.bench_function("Day 01 second task", |b| {
        b.iter(|| day.second_task(black_box(&input)))
    });
}

pub fn day22(c: &mut Criterion) {
    let input = fs::read_to_string("../input/day-22.input").unwrap();
    let day = Day22 {};
    c.bench_function("Day 22 first task", |b| {
        b.iter(|| day.first_task(black_box(&input)))
    });
    c.bench_function("Day 22 second task", |b| {
        b.iter(|| day.second_task(black_box(&input)))
    });
}

pub fn day23(c: &mut Criterion) {
    let input = fs::read_to_string("../input/day-23.input").unwrap();
    let day = Day23 {};
    c.bench_function("Day 23 first task", |b| {
        b.iter(|| day.first_task(black_box(&input)))
    });
    c.bench_function("Day 23 second task", |b| {
        b.iter(|| day.second_task(black_box(&input)))
    });
}

pub fn day24(c: &mut Criterion) {
    let input = fs::read_to_string("../input/day-24.input").unwrap();
    let day = Day24 {};
    c.bench_function("Day 24 first task", |b| {
        b.iter(|| day.first_task(black_box(&input)))
    });
    c.bench_function("Day 24 second task", |b| {
        b.iter(|| day.second_task(black_box(&input)))
    });
}

pub fn day25(c: &mut Criterion) {
    let input = fs::read_to_string("../input/day-25.input").unwrap();
    let day = Day25 {};
    c.bench_function("Day 25 first task", |b| {
        b.iter(|| day.first_task(black_box(&input)))
    });
    c.bench_function("Day 25 second task", |b| {
        b.iter(|| day.second_task(black_box(&input)))
    });
}

criterion_group!(
    benches, // day01,
    day22,
    // day23,
    // day24,
    // day25,
);
criterion_main!(benches);
