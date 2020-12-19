from aoc2020.solutions.day_19 import Rules, parse_input_messages


def test_simple_rules():
    rules_text = """0: 1 2
1: "a"
2: 1 3 | 3 1
3: "b"
"""

    rules = Rules.from_str(rules_text)

    messages = [("a", 1, True), ("b", 3, True), ("ab", 2, True),
                ("ba", 2, True), ("aab", 0, True), ("aba", 0, True),
                ("b", 1, False)]
    for message, rule_name, is_valid in messages:
        assert rules.matches_rule(message, rule_name) == is_valid


def test_interesting_rules():
    rules_text = """0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"
"""

    rules = Rules.from_str(rules_text)

    messages = [("abbbab", 0, True), ("aaaabbb", 0, False)]
    for message, rule_name, is_valid in messages:
        assert rules.matches_rule(message, rule_name) == is_valid


def test_second_task_before_changes():
    test_rules = """42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
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
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1"""

    rules = Rules.from_str(test_rules)

    messages = [
        ("abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa", False),
        ("bbabbbbaabaabba", True),
        ("babbbbaabbbbbabbbbbbaabaaabaaa", False),
        ("aaabbbbbbaaaabaababaabababbabaaabbababababaaa", False),
        ("bbbbbbbaaaabbbbaaabbabaaa", False),
        ("bbbababbbbaaaaaaaabbababaaababaabab", False),
        ("ababaaaaaabaaab", True),
        ("ababaaaaabbbaba", True),
        ("baabbaaaabbaaaababbaababb", False),
        ("abbbbabbbbaaaababbbbbbaaaababb", False),
        ("aaaaabbaabaaaaababaa", False),
        ("aaaabbaaaabbaaa", False),
        ("aaaabbaabbaaaaaaabbbabbbaaabbaabaaa", False),
        ("babaaabbbaaabaababbaabababaaab", False),
        ("aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba", False),
    ]
    for message, is_valid in messages:
        assert rules.matches_rule(message, 0) == is_valid


def test_second_task_after_changes():
    test_rules = """42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
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
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1"""

    rules = Rules.from_str(test_rules)
    rules.make_changes_in_rules()

    messages = [
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
    ]
    for message in messages:
        assert rules.matches_rule(message, 0)
