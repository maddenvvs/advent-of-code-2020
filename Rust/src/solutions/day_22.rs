use super::solution::{Error, Solution};
use itertools::Itertools;
use std::collections::{hash_map, HashSet, VecDeque};
use std::hash::{Hash, Hasher};

type Deck = VecDeque<u8>;

fn parse_deck(deck_text: &str) -> Deck {
    deck_text
        .lines()
        .skip(1) // Skip "Player ..." line
        .map(|el| el.parse().unwrap())
        .collect()
}

fn parse_decks(cards_text: &str) -> (Deck, Deck) {
    cards_text
        .split("\n\n")
        .map(parse_deck)
        .collect_tuple()
        .unwrap()
}

fn count_deck_score(deck: &Deck) -> usize {
    deck.iter()
        .rev()
        .enumerate()
        .map(|(i, &v)| (i + 1) * v as usize)
        .sum()
}

fn simulate_combat_game<'a>(
    first_deck: &'a mut Deck,
    second_deck: &'a mut Deck,
) -> (&'a Deck, &'a Deck) {
    while !first_deck.is_empty() && !second_deck.is_empty() {
        let f = first_deck.pop_front().unwrap();
        let s = second_deck.pop_front().unwrap();

        if f > s {
            first_deck.push_back(f);
            first_deck.push_back(s);
        } else {
            second_deck.push_back(s);
            second_deck.push_back(f);
        }
    }

    (first_deck, second_deck)
}

fn generate_cache_key(first_deck: &Deck, second_deck: &Deck) -> u64 {
    let mut hasher = hash_map::DefaultHasher::new();
    first_deck.hash(&mut hasher);
    second_deck.hash(&mut hasher);
    hasher.finish()
}

fn simulate_recursive_combat_game<'a>(
    first_deck: &'a mut Deck,
    second_deck: &'a mut Deck,
) -> (bool, &'a Deck, &'a Deck) {
    let mut round_cache: HashSet<u64> = HashSet::new();
    while !first_deck.is_empty() && !second_deck.is_empty() {
        let round_cache_key = generate_cache_key(first_deck, second_deck);
        if !round_cache.insert(round_cache_key) {
            return (true, first_deck, second_deck);
        }

        let f = first_deck.pop_front().unwrap() as usize;
        let s = second_deck.pop_front().unwrap() as usize;

        let mut res = f > s;

        if (f <= first_deck.len()) && (s <= second_deck.len()) {
            let mut first_sub_deck = (0..f).map(|i| first_deck[i]).collect::<Deck>();
            let mut second_sub_deck = (0..s).map(|i| second_deck[i]).collect::<Deck>();

            res = simulate_recursive_combat_game(&mut first_sub_deck, &mut second_sub_deck).0;
        }

        if res {
            first_deck.push_back(f as u8);
            first_deck.push_back(s as u8);
        } else {
            second_deck.push_back(s as u8);
            second_deck.push_back(f as u8);
        }
    }

    let game_result = !first_deck.is_empty();

    (game_result, first_deck, second_deck)
}

fn find_winning_score_in_combat(first_deck: &mut Deck, second_deck: &mut Deck) -> usize {
    let (fd, sd) = simulate_combat_game(first_deck, second_deck);

    if !fd.is_empty() {
        count_deck_score(fd)
    } else {
        count_deck_score(sd)
    }
}

fn find_winning_score_in_recursive_combat(first_deck: &mut Deck, second_deck: &mut Deck) -> usize {
    let (res, fd, sd) = simulate_recursive_combat_game(first_deck, second_deck);

    if res {
        count_deck_score(fd)
    } else {
        count_deck_score(sd)
    }
}

pub struct Day22 {}

impl Solution for Day22 {
    fn first_task(&self, cards_text: &str) -> Result<String, Error> {
        let (mut first_deck, mut second_deck) = parse_decks(cards_text);

        Ok(find_winning_score_in_combat(&mut first_deck, &mut second_deck).to_string())
    }

    fn second_task(&self, cards_text: &str) -> Result<String, Error> {
        let (mut first_deck, mut second_deck) = parse_decks(cards_text);

        Ok(find_winning_score_in_recursive_combat(&mut first_deck, &mut second_deck).to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_task() {
        let test_cards_text = "Player 1:
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
10";

        let (mut fd, mut sd) = parse_decks(test_cards_text);

        assert_eq!(find_winning_score_in_combat(&mut fd, &mut sd), 306);
    }

    #[test]
    fn test_second_task() {
        let test_cards_text = "Player 1:
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
10";

        let (mut fd, mut sd) = parse_decks(test_cards_text);

        assert_eq!(
            find_winning_score_in_recursive_combat(&mut fd, &mut sd),
            291
        );
    }
}
