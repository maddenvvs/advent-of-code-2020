use super::solution::{Error, Solution};

pub struct DoublyLinkedList {
    prev: Vec<usize>,
    next: Vec<usize>,
}

impl DoublyLinkedList {
    pub fn new(size: usize) -> Self {
        let mut prev = vec![0; size];
        let mut next = vec![0; size];

        for idx in 1..size {
            next[idx - 1] = idx;
            prev[idx] = idx - 1;
        }

        next[size - 1] = 0;
        prev[0] = size - 1;

        Self { prev, next }
    }

    fn remove_node(&mut self, node: usize) {
        self.next[self.prev[node]] = self.next[node];
        self.prev[self.next[node]] = self.prev[node];
    }

    fn insert_after(&mut self, after: usize, node: usize) {
        self.next[node] = self.next[after];
        self.next[after] = node;
        self.prev[self.next[node]] = node;
        self.prev[node] = after;
    }
}

fn parse_cups(cups_text: &str) -> Vec<u32> {
    cups_text.chars().map(|c| c.to_digit(10).unwrap()).collect()
}

fn simulate_game(cups: &[u32], moves: u32) -> Vec<u32> {
    let mut linked_list: DoublyLinkedList = DoublyLinkedList::new(cups.len());
    let mut val2node = vec![0; cups.len()];

    for (i, cup) in cups.iter().enumerate() {
        val2node[(*cup - 1) as usize] = i;
    }

    let mut curr_node = 0;

    for _ in 0..moves {
        let removed_nodes = [
            linked_list.next[curr_node],
            linked_list.next[linked_list.next[curr_node]],
            linked_list.next[linked_list.next[linked_list.next[curr_node]]],
        ];

        for &val in &removed_nodes {
            linked_list.remove_node(val);
        }

        let mut value_to_append_after = cups[curr_node] - 1;
        if value_to_append_after == 0 {
            value_to_append_after = cups.len() as u32;
        }

        while value_to_append_after == cups[removed_nodes[0]]
            || value_to_append_after == cups[removed_nodes[1]]
            || value_to_append_after == cups[removed_nodes[2]]
        {
            value_to_append_after -= 1;
            if value_to_append_after == 0 {
                value_to_append_after = cups.len() as u32;
            }
        }

        let mut node_to_insert_after = val2node[(value_to_append_after - 1) as usize];
        for &node in &removed_nodes {
            linked_list.insert_after(node_to_insert_after, node);
            node_to_insert_after = node;
        }

        curr_node = linked_list.next[curr_node];
    }

    let mut new_cups = vec![0; cups.len()];
    let mut next_node = 0;

    for new_cup in new_cups.iter_mut().take(cups.len()) {
        *new_cup = cups[next_node];
        next_node = linked_list.next[next_node];
    }

    new_cups
}

fn find_1_based_label(cups: &[u32]) -> String {
    let one_idx = cups.iter().position(|&v| v == 1).unwrap();
    let mut label = String::new();

    for i in 1..cups.len() {
        label.push_str(&cups[(one_idx + i) % cups.len()].to_string());
    }

    label
}

fn count_1_based_label_after(cups: &[u32], moves: u32) -> String {
    let new_cups = simulate_game(cups, moves);

    find_1_based_label(&new_cups)
}

fn count_product_of_two_labels_after_1(cups: &[u32]) -> u64 {
    let mut all_cups = vec![0; 1_000_000];

    for (i, &cup) in cups.iter().enumerate() {
        all_cups[i] = cup;
    }

    for (i, val) in all_cups
        .iter_mut()
        .enumerate()
        .take(1_000_000)
        .skip(cups.len())
    {
        *val = (i + 1) as u32;
    }

    let new_cups = simulate_game(&all_cups, 10_000_000);
    let one_idx = new_cups.iter().position(|&v| v == 1).unwrap();

    (new_cups[(one_idx + 1) % new_cups.len()] as u64)
        * (new_cups[(one_idx + 2) % new_cups.len()] as u64)
}

pub struct Day23 {}

impl Solution for Day23 {
    fn first_task(&self, cups_text: &str) -> Result<String, Error> {
        let cups = parse_cups(cups_text);

        Ok(count_1_based_label_after(&cups, 100))
    }

    fn second_task(&self, cups_text: &str) -> Result<String, Error> {
        let cups = parse_cups(cups_text);

        Ok(count_product_of_two_labels_after_1(&cups).to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test_1_based_label() {
        let test_cups_text = "389125467";
        let test_cups = parse_cups(test_cups_text);

        assert_eq!(find_1_based_label(&test_cups), "25467389");
    }

    #[test]
    fn test_some_next_moves() {
        let test_cups_text = "389125467";
        let test_cups = parse_cups(test_cups_text);

        for (moves, label) in &[(1, "54673289"), (2, "32546789"), (3, "34672589")] {
            assert_eq!(count_1_based_label_after(&test_cups, *moves as u32), *label);
        }
    }

    #[test]
    fn test_first_task() {
        let test_cups_text = "389125467";
        let test_cups = parse_cups(test_cups_text);

        assert_eq!(count_1_based_label_after(&test_cups, 10), "92658374");
        assert_eq!(count_1_based_label_after(&test_cups, 100), "67384529");
    }

    #[test]
    fn test_second_task() {
        let test_cups_text = "389125467";
        let test_cups = parse_cups(test_cups_text);

        assert_eq!(
            count_product_of_two_labels_after_1(&test_cups),
            149245887792_u64
        );
    }
}
