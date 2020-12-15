use super::challenge::{Challenge, ChallengeErr};
use std::collections::HashMap;

enum Command<'a> {
    Mask { mask: &'a str },
    MemorySet { addr: &'a str, value: i64 },
}

impl Command<'_> {
    fn from_str(s: &str) -> Command {
        let mut parts = s.split(" = ");

        match parts.next().unwrap() {
            "mask" => Command::Mask {
                mask: parts.next().unwrap(),
            },
            val => Command::MemorySet {
                addr: &val[4..val.len() - 1],
                value: parts.next().unwrap().parse().unwrap(),
            },
        }
    }
}

struct TrieNode {
    left: Option<Box<TrieNode>>,
    right: Option<Box<TrieNode>>,
    value: i64,
}

impl TrieNode {
    fn new() -> TrieNode {
        TrieNode {
            left: None,
            right: None,
            value: 0,
        }
    }

    fn add_value_at(&mut self, address: &str, value: i64, position: usize) {
        if position >= address.chars().count() {
            self.value = value;
            return;
        }

        match address.chars().nth(position).unwrap() {
            '1' => {
                if self.right.is_none() {
                    self.right = Some(Box::new(TrieNode::new()));
                }

                self.right
                    .as_mut()
                    .unwrap()
                    .add_value_at(address, value, position + 1);
            }
            '0' => {
                if self.left.is_none() {
                    self.left = Some(Box::new(TrieNode::new()));
                }

                self.left
                    .as_mut()
                    .unwrap()
                    .add_value_at(address, value, position + 1);
            }
            'X' => {
                if self.left.is_none() {
                    self.left = Some(Box::new(TrieNode::new()));
                }
                if self.right.is_none() {
                    self.right = Some(Box::new(TrieNode::new()));
                }

                self.left
                    .as_mut()
                    .unwrap()
                    .add_value_at(address, value, position + 1);
                self.right
                    .as_mut()
                    .unwrap()
                    .add_value_at(address, value, position + 1);
            }
            _ => (),
        };

        self.value = match (self.left.as_mut(), self.right.as_mut()) {
            (Some(l), Some(r)) => l.value + r.value,
            (_, Some(r)) => r.value,
            (Some(l), _) => l.value,
            _ => value,
        };
    }
}

fn parse_program(program_text: &str) -> Vec<Command> {
    program_text.lines().map(Command::from_str).collect()
}

fn evaluate_program_v1(program: &[Command]) -> i64 {
    use Command::*;

    let mut memory: HashMap<i64, i64> = HashMap::new();
    let mut mask = "";

    for command in program {
        match command {
            Mask { mask: m } => mask = m,
            MemorySet { addr, value } => {
                let mut value = *value;
                let addr: i64 = addr.parse().unwrap();

                for (i, v) in mask.chars().rev().enumerate() {
                    match v {
                        '1' => value |= 1 << i,
                        '0' => value &= !(1 << i),
                        _ => (),
                    };
                }

                memory.insert(addr, value);
            }
        }
    }

    memory.values().sum()
}

fn apply_mask(mask: &str, addr: &str) -> String {
    let mut res = String::new();

    for (m, v) in mask.chars().zip(addr.chars()) {
        match m {
            'X' => res.push(m),
            '1' => res.push('1'),
            _ => res.push(v),
        };
    }

    res
}

fn evaluate_program_v2(program: &[Command]) -> i64 {
    use Command::*;

    let mut memory = TrieNode::new();
    let mut mask = "";

    for command in program {
        match command {
            Mask { mask: m } => mask = m,
            MemorySet { addr, value } => {
                let addr = addr.parse::<i64>().unwrap();
                let address = apply_mask(&mask, &format!("{:0>36b}", addr));

                memory.add_value_at(&address, *value, 0);
            }
        }
    }

    memory.value
}

fn find_memory_values_sum_v1(program: &[Command]) -> i64 {
    evaluate_program_v1(program)
}

fn find_memory_values_sum_v2(program: &[Command]) -> i64 {
    evaluate_program_v2(program)
}

pub struct Solution {}

impl Challenge for Solution {
    fn first_part(&self, program_text: &str) -> Result<String, ChallengeErr> {
        let program = parse_program(program_text);

        Ok(find_memory_values_sum_v1(&program).to_string())
    }

    fn second_part(&self, program_text: &str) -> Result<String, ChallengeErr> {
        let program = parse_program(program_text);

        Ok(find_memory_values_sum_v2(&program).to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_sum_v1() {
        let test_program_1_text = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

        let test_program_1 = parse_program(&test_program_1_text);
        assert_eq!(find_memory_values_sum_v1(&test_program_1), 165);
    }

    #[test]
    fn test_memory_sum_v2() {
        let test_program_2_text = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";

        let test_program_2 = parse_program(&test_program_2_text);
        assert_eq!(find_memory_values_sum_v2(&test_program_2), 208);
    }
}
