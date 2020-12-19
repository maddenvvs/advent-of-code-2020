use super::solution::{Error, Solution};
use std::collections::HashMap;

#[derive(Debug)]
enum Match {
    Letter { letter: char },
    RuleName { name: i32 },
    And { rules: Vec<Match> },
    Or { rules: Vec<Match> },
}

impl Match {
    fn try_match(&self, message: &Vec<char>, idx: usize, rules: &Rules) -> Vec<usize> {
        use Match::*;

        if idx >= message.len() {
            return vec![];
        }

        match self {
            Letter { letter } => {
                if *letter == message[idx] {
                    vec![idx + 1]
                } else {
                    vec![]
                }
            }

            RuleName { name } => rules
                .graph
                .get(name)
                .unwrap()
                .try_match(message, idx, rules),

            And { rules: r } => r.iter().fold(vec![idx], |acc, el| {
                acc.iter()
                    .flat_map(|&i| el.try_match(message, i, rules))
                    .collect()
            }),

            Or { rules: r } => r
                .iter()
                .flat_map(|rule| rule.try_match(message, idx, rules))
                .collect(),
        }
    }
}

struct Rules {
    graph: HashMap<i32, Match>,
}

impl Rules {
    fn add_rule(&mut self, name: i32, rule: Match) {
        self.graph.insert(name, rule);
    }

    fn matches_rule(&self, message: &str, rule: i32) -> bool {
        let message: Vec<char> = message.chars().collect();
        let idxs = self.graph.get(&rule).unwrap().try_match(&message, 0, self);

        idxs.contains(&message.len())
    }

    fn make_changes_in_rules(&mut self) {
        use Match::*;

        self.graph.insert(
            8,
            Or {
                rules: vec![
                    RuleName { name: 42 },
                    And {
                        rules: vec![RuleName { name: 42 }, RuleName { name: 8 }],
                    },
                ],
            },
        );

        self.graph.insert(
            11,
            Or {
                rules: vec![
                    And {
                        rules: vec![RuleName { name: 42 }, RuleName { name: 31 }],
                    },
                    And {
                        rules: vec![
                            RuleName { name: 42 },
                            RuleName { name: 11 },
                            RuleName { name: 31 },
                        ],
                    },
                ],
            },
        );
    }

    fn new(s: &str) -> Rules {
        let mut rules: Rules = Rules {
            graph: HashMap::new(),
        };

        for (name, rule) in s.lines().map(parse_rule) {
            rules.add_rule(name, rule);
        }

        rules
    }
}

fn parse_definition(definition_text: &str) -> Match {
    let parts: Vec<&str> = definition_text.split_whitespace().collect();
    if parts.len() > 1 {
        return Match::And {
            rules: parts
                .iter()
                .map(|num| Match::RuleName {
                    name: num.parse().unwrap(),
                })
                .collect(),
        };
    }

    if parts[0].starts_with("\"") {
        return Match::Letter {
            letter: parts[0].chars().nth(1).unwrap(),
        };
    }

    Match::RuleName {
        name: parts[0].parse().unwrap(),
    }
}

fn parse_rule(rule_text: &str) -> (i32, Match) {
    let mut parts = rule_text.split(": ");
    let name = parts.next().unwrap().parse::<i32>().unwrap();
    let definition: Vec<&str> = parts.next().unwrap().split(" | ").collect();

    if definition.len() > 1 {
        return (
            name,
            Match::Or {
                rules: definition.iter().map(|el| parse_definition(el)).collect(),
            },
        );
    }

    (name, parse_definition(definition[0]))
}

fn parse_input_messages(input_messages: &str) -> (Rules, Vec<&str>) {
    let mut parts = input_messages.split("\n\n");
    let rules = Rules::new(parts.next().unwrap());
    let messages = parts.next().unwrap().lines().collect();

    (rules, messages)
}

fn count_messages_match_rule_0(rules: &Rules, messages: &Vec<&str>) -> usize {
    messages.iter().filter(|m| rules.matches_rule(m, 0)).count()
}

pub struct Day19 {}

impl Solution for Day19 {
    fn first_task(&self, messages_text: &str) -> Result<String, Error> {
        let (rules, messages) = parse_input_messages(messages_text);

        Ok(count_messages_match_rule_0(&rules, &messages).to_string())
    }

    fn second_task(&self, messages_text: &str) -> Result<String, Error> {
        let (mut rules, messages) = parse_input_messages(messages_text);
        rules.make_changes_in_rules();

        Ok(count_messages_match_rule_0(&rules, &messages).to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_rules() {
        let rules_text = "0: 1 2
1: \"a\"
2: 1 3 | 3 1
3: \"b\"
";

        let rules = Rules::new(&rules_text);

        let messages = [
            ("a", 1, true),
            ("b", 3, true),
            ("ab", 2, true),
            ("ba", 2, true),
            ("aab", 0, true),
            ("aba", 0, true),
            ("b", 1, false),
        ];
        for (message, rule_name, is_valid) in &messages {
            assert_eq!(rules.matches_rule(message, *rule_name), *is_valid);
        }
    }

    #[test]
    fn test_interesting_rules() {
        let rules_text = "0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"
";

        let rules = Rules::new(rules_text);

        let messages = [("abbbab", 0, true), ("aaaabbb", 0, false)];
        for (message, rule_name, is_valid) in &messages {
            assert_eq!(rules.matches_rule(message, *rule_name), *is_valid);
        }
    }

    #[test]
    fn test_second_task_before_changes() {
        let test_rules = "42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1";

        let rules = Rules::new(test_rules);

        let messages = [
            ("abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa", false),
            ("bbabbbbaabaabba", true),
            ("babbbbaabbbbbabbbbbbaabaaabaaa", false),
            ("aaabbbbbbaaaabaababaabababbabaaabbababababaaa", false),
            ("bbbbbbbaaaabbbbaaabbabaaa", false),
            ("bbbababbbbaaaaaaaabbababaaababaabab", false),
            ("ababaaaaaabaaab", true),
            ("ababaaaaabbbaba", true),
            ("baabbaaaabbaaaababbaababb", false),
            ("abbbbabbbbaaaababbbbbbaaaababb", false),
            ("aaaaabbaabaaaaababaa", false),
            ("aaaabbaaaabbaaa", false),
            ("aaaabbaabbaaaaaaabbbabbbaaabbaabaaa", false),
            ("babaaabbbaaabaababbaabababaaab", false),
            ("aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba", false),
        ];
        for (message, is_valid) in &messages {
            assert_eq!(rules.matches_rule(message, 0), *is_valid);
        }
    }

    #[test]
    fn test_second_task_after_changes() {
        let test_rules = "42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1";

        let mut rules = Rules::new(test_rules);
        rules.make_changes_in_rules();

        let messages = [
            "bbabbbbaabaabba",
            "babbbbaabbbbbabbbbbbaabaaabaaa",
            "aaabbbbbbaaaabaababaabababbabaaabbababababaaa",
            "bbbbbbbaaaabbbbaaabbabaaa",
            "bbbababbbbaaaaaaaabbababaaababaabab",
            "ababaaaaaabaaab",
            "ababaaaaabbbaba",
            "baabbaaaabbaaaababbaababb",
            "abbbbabbbbaaaababbbbbbaaaababb",
            "aaaaabbaabaaaaababaa",
            "aaaabbaabbaaaaaaabbbabbbaaabbaabaaa",
            "aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba",
        ];
        for message in &messages {
            assert_eq!(rules.matches_rule(message, 0), true);
        }
    }
}
