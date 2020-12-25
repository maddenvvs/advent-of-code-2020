use super::solution::{Error, Solution};

fn parse_cups(cups_text: &str) -> Vec<usize> {
    cups_text
        .chars()
        .map(|c| c.to_digit(10).map(|c| c as usize).unwrap())
        .collect()
}

fn build_cups_buffer(cups: &[usize], size: usize) -> Vec<usize> {
    let mut buffer = vec![0; size + 1];
    let last = (1..size).fold(cups[0], |curr, i| {
        buffer[curr] = if i < cups.len() { cups[i] } else { i + 1 };
        buffer[curr]
    });
    buffer[last] = cups[0];
    buffer
}

fn simulate_move(cups_buffer: &mut [usize], current_cup: usize) -> usize {
    let size = cups_buffer.len() - 1;
    let (a, b, c) = (
        cups_buffer[current_cup],
        cups_buffer[cups_buffer[current_cup]],
        cups_buffer[cups_buffer[cups_buffer[current_cup]]],
    );

    let mut destination = if current_cup == 1 {
        size
    } else {
        current_cup - 1
    };

    while destination == a || destination == b || destination == c {
        destination = if destination == 1 {
            size
        } else {
            destination - 1
        };
    }

    cups_buffer[current_cup] = cups_buffer[c];
    cups_buffer[c] = cups_buffer[destination];
    cups_buffer[destination] = a;

    cups_buffer[current_cup]
}

fn simulate_game(start_cup: usize, buffer: &mut [usize], moves: usize) {
    (0..moves).fold(start_cup, |curr, _| simulate_move(buffer, curr));
}

fn find_1_based_label(buffer: &[usize]) -> String {
    let mut one_idx = 1;
    let mut label = String::new();

    for _i in 1..buffer.len() - 1 {
        label.push_str(&buffer[one_idx].to_string());
        one_idx = buffer[one_idx];
    }

    label
}

fn count_1_based_label_after(cups: &[usize], moves: usize) -> String {
    let mut buffer = build_cups_buffer(&cups, cups.len());
    simulate_game(cups[0], &mut buffer, moves);

    find_1_based_label(&buffer)
}

fn count_product_of_two_labels_after_1(cups: &[usize]) -> usize {
    let mut buffer = build_cups_buffer(&cups, 1_000_000);
    simulate_game(cups[0], &mut buffer, 10_000_000);

    buffer[1] * buffer[buffer[1]]
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
        let test_buffer = build_cups_buffer(&test_cups, test_cups.len());

        assert_eq!(find_1_based_label(&test_buffer), "25467389");
    }

    #[test]
    fn test_some_next_moves() {
        let test_cups_text = "389125467";
        let test_cups = parse_cups(test_cups_text);

        for (moves, label) in &[(1, "54673289"), (2, "32546789"), (3, "34672589")] {
            assert_eq!(
                count_1_based_label_after(&test_cups, *moves as usize),
                *label
            );
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
            149245887792
        );
    }
}
