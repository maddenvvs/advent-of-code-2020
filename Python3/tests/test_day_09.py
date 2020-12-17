from aoc2020.solutions.day_09 import parse_cypher, \
    find_first_incorrect_cypher_number, \
    find_encryption_weakness_of


def test_tasks() -> None:
    test_cypher_text = """35
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
576"""

    test_cypher = parse_cypher(test_cypher_text)

    assert find_first_incorrect_cypher_number(
        test_cypher, preamble_len=5) == 127
    assert find_encryption_weakness_of(test_cypher, preamble_len=5) == 62
