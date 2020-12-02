use std::collections::HashSet;
use std::fs;

const NEW_YEAR: i32 = 2020;

fn find_product_of_three_numbers_equal_to_2020(entities: &Vec<i32>) -> i32 {
    let mut entities_copy = entities.to_vec();
    entities_copy.sort();

    for f in 0..(entities.len() - 2) {
        let target_sum = NEW_YEAR - entities_copy[f];
        if target_sum <= 0 {
            continue;
        }

        let mut l = f + 1;
        let mut r = entities.len() - 1;

        while l < r {
            let temp_sum = entities_copy[l] + entities_copy[r];

            if temp_sum == target_sum {
                return entities_copy[f] * entities_copy[l] * entities_copy[r];
            }

            if temp_sum < target_sum {
                l += 1;
            } else {
                r -= 1;
            }
        }
    }

    return 0;
}

fn find_product_of_two_numbers_equal_to_2020(entities: &Vec<i32>) -> i32 {
    let mut seen = HashSet::new();

    for entity in entities {
        let candidate = NEW_YEAR - entity;
        if seen.contains(&candidate) {
            return candidate * entity;
        }

        seen.insert(entity);
    }

    return 0;
}

fn task_tests() {
    let test_entries = vec![1721, 979, 366, 299, 675, 1456];

    assert_eq!(
        find_product_of_two_numbers_equal_to_2020(&test_entries),
        514579
    );
    assert_eq!(
        find_product_of_three_numbers_equal_to_2020(&test_entries),
        241861950
    );
}

fn run_tasks() {
    let entities: Vec<i32> = fs::read_to_string("input/day-1.input")
        .unwrap()
        .lines()
        .map(|s| s.parse().unwrap())
        .collect();

    println!(
        "Day 1-1: {}",
        find_product_of_two_numbers_equal_to_2020(&entities)
    );
    println!(
        "Day 1-2: {}",
        find_product_of_three_numbers_equal_to_2020(&entities)
    );
}

fn main() {
    task_tests();
    run_tasks();
}
