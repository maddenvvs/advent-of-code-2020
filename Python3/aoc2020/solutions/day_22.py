from typing import List, Tuple, Dict

from .solution import Solution

Deck = List[int]


def parse_decks(cards_text: str) -> Tuple[Deck, Deck]:
    player1, player2 = cards_text.split("\n\n")
    player1_deck = player1.splitlines()[1:]
    player2_deck = player2.splitlines()[1:]

    return [int(i, base=10) for i in player1_deck], [int(i, base=10) for i in player2_deck]


def simulate_combat_game(first_deck: Deck, second_deck: Deck) -> Tuple[Deck, Deck]:
    while first_deck and second_deck:
        f = first_deck.pop(0)
        s = second_deck.pop(0)

        if f > s:
            first_deck.append(f)
            first_deck.append(s)
        else:
            second_deck.append(s)
            second_deck.append(f)

    return first_deck, second_deck


def generate_cache_key(d1: Deck, d2: Deck) -> str:
    return f'{",".join(map(str,d1))}|{",".join(map(str,d2))}'


def simulate_recursive_combat_game(
        first_deck: Deck,
        second_deck: Deck,
        game_cache: Dict[str, bool]
) -> Tuple[bool, Deck, Deck]:
    cache_key = generate_cache_key(first_deck, second_deck)

    if cache_key in game_cache:
        return game_cache[cache_key], first_deck, second_deck

    round_cache = set()
    while first_deck and second_deck:
        round_cache_key = generate_cache_key(first_deck, second_deck)
        if round_cache_key in round_cache:
            return True, first_deck, second_deck

        round_cache.add(round_cache_key)

        f = first_deck.pop(0)
        s = second_deck.pop(0)

        res = f > s

        if f <= len(first_deck) and s <= len(second_deck):
            res, _, _ = simulate_recursive_combat_game(
                first_deck[:f], second_deck[:s], game_cache)

        if res:
            first_deck.append(f)
            first_deck.append(s)
        else:
            second_deck.append(s)
            second_deck.append(f)

    game_result = bool(first_deck)
    game_cache[cache_key] = game_result

    return game_result, first_deck, second_deck


def count_deck_score(deck: Deck) -> int:
    return sum(i*v for i, v in enumerate(reversed(deck), start=1))


def find_winning_score_in_combat(player1_deck: Deck, player2_deck: Deck) -> int:
    p1_deck, p2_deck = simulate_combat_game(player1_deck, player2_deck)

    return count_deck_score(p1_deck) if p1_deck else count_deck_score(p2_deck)


def find_winning_score_in_recursive_combat(player1_deck: Deck, player2_deck: Deck) -> int:
    res, p1_deck, p2_deck = simulate_recursive_combat_game(
        player1_deck, player2_deck, {})

    return count_deck_score(p1_deck) if res else count_deck_score(p2_deck)


class Day22(Solution):

    def first_task(self, cards_text: str) -> str:
        decks = parse_decks(cards_text)

        return str(find_winning_score_in_combat(*decks))

    def second_task(self, cards_text: str) -> str:
        decks = parse_decks(cards_text)

        return str(find_winning_score_in_recursive_combat(*decks))
