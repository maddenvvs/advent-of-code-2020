use super::challenge::{Challenge, Error as ChallengeErr};
use std::collections::{HashMap, HashSet};

const SHINY_GOLD: &str = "shiny gold";

#[derive(Debug)]
struct BagsRules {
    graph: HashMap<String, Vec<(String, i32)>>,
}

impl BagsRules {
    fn reverse(&self) -> BagsRules {
        let mut reversed_graph = BagsRules {
            graph: HashMap::new(),
        };

        for (u, others) in self.graph.iter() {
            for (v, c) in others {
                reversed_graph
                    .graph
                    .entry(v.to_string())
                    .or_insert(vec![])
                    .push((u.to_string(), *c));
            }
        }

        reversed_graph
    }

    fn count_nodes_reachable_from(&self, bag: &String) -> i32 {
        let mut stack = vec![bag];
        let mut visited = HashSet::new();
        visited.insert(bag);
        let mut count = 0;

        while !stack.is_empty() {
            let curr = stack.pop().unwrap();
            count += 1;

            if !self.graph.contains_key(curr) {
                continue;
            }

            for (v, _) in &self.graph[curr] {
                if !visited.contains(&v) {
                    visited.insert(&v);
                    stack.push(&v);
                }
            }
        }

        count
    }

    fn count_bag_colors_containing(&self, bag: &str) -> i32 {
        self.reverse()
            .count_nodes_reachable_from(&String::from(bag))
            - 1
    }

    fn count_bags_inside(&self, bag: &str) -> i32 {
        self.graph[bag]
            .iter()
            .fold(0, |acc, (o, c)| acc + c * (self.count_bags_inside(&o) + 1))
    }

    fn from_rules_text(rules_text: &str) -> BagsRules {
        let mut bags_rules = BagsRules {
            graph: HashMap::new(),
        };

        for bag_rule in rules_text.lines() {
            let (container_bag, bags_inside) = BagsRules::parse_bag_rule(bag_rule);
            let graph_entry = bags_rules.graph.entry(container_bag).or_insert(vec![]);
            for (bag_inside, count) in bags_inside {
                graph_entry.push((bag_inside, count));
            }
        }

        bags_rules
    }

    fn parse_bag_rule(bag_rule: &str) -> (String, Vec<(String, i32)>) {
        let words: Vec<&str> = bag_rule.split_ascii_whitespace().collect();
        let container_bag: String = format!("{} {}", words[0], words[1]);
        let mut bags_inside: Vec<(String, i32)> = vec![];

        if words[4] != "no" {
            for i in (4..words.len()).step_by(4) {
                let count: i32 = words[i].parse().unwrap();
                let other_bag: String = format!("{} {}", words[i + 1], words[i + 2]);
                bags_inside.push((other_bag, count));
            }
        }

        (container_bag, bags_inside)
    }
}

pub struct Solution {}

impl Challenge for Solution {
    fn first_part(&self, bags_rules_text: &str) -> Result<String, ChallengeErr> {
        let bags_rules = BagsRules::from_rules_text(&bags_rules_text);

        Ok(bags_rules
            .count_bag_colors_containing(SHINY_GOLD)
            .to_string())
    }

    fn second_part(&self, bags_rules_text: &str) -> Result<String, ChallengeErr> {
        let bags_rules = BagsRules::from_rules_text(&bags_rules_text);

        Ok(bags_rules.count_bags_inside(SHINY_GOLD).to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_count_bag_colors_containing() {
        let test_bags_rules_1 = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

        let bags_rules = BagsRules::from_rules_text(test_bags_rules_1);

        assert_eq!(bags_rules.count_bag_colors_containing(SHINY_GOLD), 4);
    }

    #[test]
    fn example_count_bags_inside_1() {
        let test_bags_rules_1 = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

        let bags_rules = BagsRules::from_rules_text(test_bags_rules_1);

        assert_eq!(bags_rules.count_bags_inside(SHINY_GOLD), 32);
    }

    #[test]
    fn example_count_bags_inside_2() {
        let test_bags_rules_2 = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

        assert_eq!(
            BagsRules::from_rules_text(test_bags_rules_2).count_bags_inside(SHINY_GOLD),
            126
        );
    }
}
