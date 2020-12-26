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

pub fn day02(c: &mut Criterion) {
    let input = fs::read_to_string("../input/day-02.input").unwrap();
    let day = Day02 {};
    c.bench_function("Day 02 first task", |b| {
        b.iter(|| day.first_task(black_box(&input)))
    });
    c.bench_function("Day 02 second task", |b| {
        b.iter(|| day.second_task(black_box(&input)))
    });
}

pub fn day03(c: &mut Criterion) {
    let input = fs::read_to_string("../input/day-03.input").unwrap();
    let day = Day03 {};
    c.bench_function("Day 03 first task", |b| {
        b.iter(|| day.first_task(black_box(&input)))
    });
    c.bench_function("Day 03 second task", |b| {
        b.iter(|| day.second_task(black_box(&input)))
    });
}

pub fn day04(c: &mut Criterion) {
    let input = fs::read_to_string("../input/day-04.input").unwrap();
    let day = Day04 {};
    c.bench_function("Day 04 first task", |b| {
        b.iter(|| day.first_task(black_box(&input)))
    });
    c.bench_function("Day 04 second task", |b| {
        b.iter(|| day.second_task(black_box(&input)))
    });
}

pub fn day05(c: &mut Criterion) {
    let input = fs::read_to_string("../input/day-05.input").unwrap();
    let day = Day05 {};
    c.bench_function("Day 05 first task", |b| {
        b.iter(|| day.first_task(black_box(&input)))
    });
    c.bench_function("Day 05 second task", |b| {
        b.iter(|| day.second_task(black_box(&input)))
    });
}

pub fn day06(c: &mut Criterion) {
    let input = fs::read_to_string("../input/day-06.input").unwrap();
    let day = Day06 {};
    c.bench_function("Day 06 first task", |b| {
        b.iter(|| day.first_task(black_box(&input)))
    });
    c.bench_function("Day 06 second task", |b| {
        b.iter(|| day.second_task(black_box(&input)))
    });
}

pub fn day07(c: &mut Criterion) {
    let input = fs::read_to_string("../input/day-07.input").unwrap();
    let day = Day07 {};
    c.bench_function("Day 07 first task", |b| {
        b.iter(|| day.first_task(black_box(&input)))
    });
    c.bench_function("Day 07 second task", |b| {
        b.iter(|| day.second_task(black_box(&input)))
    });
}

pub fn day08(c: &mut Criterion) {
    let input = fs::read_to_string("../input/day-08.input").unwrap();
    let day = Day08 {};
    c.bench_function("Day 08 first task", |b| {
        b.iter(|| day.first_task(black_box(&input)))
    });
    c.bench_function("Day 08 second task", |b| {
        b.iter(|| day.second_task(black_box(&input)))
    });
}

pub fn day09(c: &mut Criterion) {
    let input = fs::read_to_string("../input/day-09.input").unwrap();
    let day = Day09 {};
    c.bench_function("Day 09 first task", |b| {
        b.iter(|| day.first_task(black_box(&input)))
    });
    c.bench_function("Day 09 second task", |b| {
        b.iter(|| day.second_task(black_box(&input)))
    });
}

pub fn day10(c: &mut Criterion) {
    let input = fs::read_to_string("../input/day-10.input").unwrap();
    let day = Day10 {};
    c.bench_function("Day 10 first task", |b| {
        b.iter(|| day.first_task(black_box(&input)))
    });
    c.bench_function("Day 10 second task", |b| {
        b.iter(|| day.second_task(black_box(&input)))
    });
}

pub fn day11(c: &mut Criterion) {
    let input = fs::read_to_string("../input/day-11.input").unwrap();
    let day = Day11 {};
    c.bench_function("Day 11 first task", |b| {
        b.iter(|| day.first_task(black_box(&input)))
    });
    c.bench_function("Day 11 second task", |b| {
        b.iter(|| day.second_task(black_box(&input)))
    });
}

pub fn day12(c: &mut Criterion) {
    let input = fs::read_to_string("../input/day-12.input").unwrap();
    let day = Day12 {};
    c.bench_function("Day 12 first task", |b| {
        b.iter(|| day.first_task(black_box(&input)))
    });
    c.bench_function("Day 12 second task", |b| {
        b.iter(|| day.second_task(black_box(&input)))
    });
}

pub fn day13(c: &mut Criterion) {
    let input = fs::read_to_string("../input/day-13.input").unwrap();
    let day = Day13 {};
    c.bench_function("Day 13 first task", |b| {
        b.iter(|| day.first_task(black_box(&input)))
    });
    c.bench_function("Day 13 second task", |b| {
        b.iter(|| day.second_task(black_box(&input)))
    });
}

pub fn day14(c: &mut Criterion) {
    let input = fs::read_to_string("../input/day-14.input").unwrap();
    let day = Day14 {};
    c.bench_function("Day 14 first task", |b| {
        b.iter(|| day.first_task(black_box(&input)))
    });
    c.bench_function("Day 14 second task", |b| {
        b.iter(|| day.second_task(black_box(&input)))
    });
}

pub fn day15(c: &mut Criterion) {
    let input = fs::read_to_string("../input/day-15.input").unwrap();
    let day = Day15 {};
    c.bench_function("Day 15 first task", |b| {
        b.iter(|| day.first_task(black_box(&input)))
    });
    c.bench_function("Day 15 second task", |b| {
        b.iter(|| day.second_task(black_box(&input)))
    });
}

pub fn day16(c: &mut Criterion) {
    let input = fs::read_to_string("../input/day-16.input").unwrap();
    let day = Day16 {};
    c.bench_function("Day 16 first task", |b| {
        b.iter(|| day.first_task(black_box(&input)))
    });
    c.bench_function("Day 16 second task", |b| {
        b.iter(|| day.second_task(black_box(&input)))
    });
}

pub fn day17(c: &mut Criterion) {
    let input = fs::read_to_string("../input/day-17.input").unwrap();
    let day = Day17 {};
    c.bench_function("Day 17 first task", |b| {
        b.iter(|| day.first_task(black_box(&input)))
    });
    c.bench_function("Day 17 second task", |b| {
        b.iter(|| day.second_task(black_box(&input)))
    });
}

pub fn day18(c: &mut Criterion) {
    let input = fs::read_to_string("../input/day-18.input").unwrap();
    let day = Day18 {};
    c.bench_function("Day 18 first task", |b| {
        b.iter(|| day.first_task(black_box(&input)))
    });
    c.bench_function("Day 18 second task", |b| {
        b.iter(|| day.second_task(black_box(&input)))
    });
}

pub fn day19(c: &mut Criterion) {
    let input = fs::read_to_string("../input/day-19.input").unwrap();
    let day = Day19 {};
    c.bench_function("Day 19 first task", |b| {
        b.iter(|| day.first_task(black_box(&input)))
    });
    c.bench_function("Day 19 second task", |b| {
        b.iter(|| day.second_task(black_box(&input)))
    });
}

pub fn day20(c: &mut Criterion) {
    let input = fs::read_to_string("../input/day-20.input").unwrap();
    let day = Day20 {};
    c.bench_function("Day 20 first task", |b| {
        b.iter(|| day.first_task(black_box(&input)))
    });
    c.bench_function("Day 20 second task", |b| {
        b.iter(|| day.second_task(black_box(&input)))
    });
}

pub fn day21(c: &mut Criterion) {
    let input = fs::read_to_string("../input/day-21.input").unwrap();
    let day = Day21 {};
    c.bench_function("Day 21 first task", |b| {
        b.iter(|| day.first_task(black_box(&input)))
    });
    c.bench_function("Day 21 second task", |b| {
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
    benches,
    day01,
    // day02,
    // day03,
    // day04,
    // day05,
    // day06,
    // day07,
    // day08,
    // day09,
    // day10,
    // day11,
    // day12,
    // day13,
    // day14,
    // day15,
    // day16,
    // day17,
    // day18,
    // day19,
    // day20,
    // day21,
    // day22,
    // day23,
    // day24,
    // day25
);
criterion_main!(benches);
