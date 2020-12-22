from aoc2020.solutions.day_22 import parse_decks, \
    find_winning_score_in_combat, \
    find_winning_score_in_recursive_combat


def test_first_task():
    test_cards_text = """Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10"""

    decks = parse_decks(test_cards_text)

    assert find_winning_score_in_combat(*decks) == 306


def test_second_task():
    test_cards_text = """Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10"""

    decks = parse_decks(test_cards_text)

    assert find_winning_score_in_recursive_combat(*decks) == 291
