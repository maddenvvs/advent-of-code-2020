#![warn(clippy::all)]

use std::collections::{HashMap, VecDeque};
use std::fs;

fn parse_cypher(cypher_text: &str) -> Vec<u64> {
    cypher_text.lines().map(|el| el.parse().unwrap()).collect()
}

fn find_first_incorrect_cypher_number(cypher: &[u64], preamble: usize) -> Option<u64> {
    let mut unique_sums_counter: HashMap<u64, u64> = HashMap::new();
    let mut l = 0;

    for (r, new_num) in cypher.iter().take(preamble).enumerate() {
        for existing_num in cypher.iter().take(r) {
            let occurences = unique_sums_counter
                .entry(*new_num + existing_num)
                .or_insert(0);
            *occurences += 1;
        }
    }

    for (r, new_num) in cypher.iter().skip(preamble).enumerate() {
        let new_num_entry = unique_sums_counter.entry(*new_num).or_insert(0);
        if *new_num_entry == 0 {
            return Some(*new_num);
        }

        let num_to_remove = cypher[l];

        for existing_num in &cypher[l + 1..r + preamble] {
            let old_summa_entry = unique_sums_counter
                .entry(num_to_remove + existing_num)
                .or_insert(1);
            *old_summa_entry -= 1;

            let new_summa_entry = unique_sums_counter
                .entry(*new_num + existing_num)
                .or_insert(0);
            *new_summa_entry += 1;
        }

        l += 1;
    }

    None
}

fn find_ecryption_weakness_value(cypher: &[u64], target_sum: u64) -> Option<u64> {
    let mut l = 0;
    let mut temp_sum = 0;
    let mut min_deque: VecDeque<u64> = VecDeque::new();
    let mut max_deque: VecDeque<u64> = VecDeque::new();

    for next_num in cypher.iter() {
        temp_sum += next_num;

        while temp_sum > target_sum {
            let number_to_remove = cypher[l];

            temp_sum -= number_to_remove;

            if !min_deque.is_empty() && min_deque[0] == number_to_remove {
                min_deque.pop_front();
            }

            if !max_deque.is_empty() && max_deque[0] == number_to_remove {
                max_deque.pop_front();
            }

            l += 1;
        }

        while !min_deque.is_empty() && min_deque.back().unwrap() > next_num {
            min_deque.pop_back();
        }
        min_deque.push_back(*next_num);

        while !max_deque.is_empty() && max_deque.back().unwrap() < next_num {
            max_deque.pop_back();
        }
        max_deque.push_back(*next_num);

        if temp_sum == target_sum {
            return Some(min_deque[0] + max_deque[0]);
        }
    }

    None
}

fn find_encryption_weakness_of(cypher: &[u64], preamble: usize) -> Option<u64> {
    find_first_incorrect_cypher_number(cypher, preamble)
        .and_then(|val| find_ecryption_weakness_value(cypher, val))
}

fn test_tasks() {
    let test_cypher_text = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

    let test_cypher = parse_cypher(&test_cypher_text);

    assert_eq!(
        find_first_incorrect_cypher_number(&test_cypher, 5),
        Some(127_u64)
    );

    assert_eq!(find_encryption_weakness_of(&test_cypher, 5), Some(62_u64));
}

fn run_tasks() {
    let cypher_text = fs::read_to_string("input/day-9.input").expect("Missing cypher text");
    let cypher = parse_cypher(&cypher_text);

    if let Some(incorrect_number) = find_first_incorrect_cypher_number(&cypher, 25) {
        println!("Day 9-1: {}", incorrect_number);
    }

    if let Some(encryption_weakness) = find_encryption_weakness_of(&cypher, 25) {
        println!("Day 9-2: {}", encryption_weakness);
    }
}

fn main() {
    test_tasks();
    run_tasks();
}
