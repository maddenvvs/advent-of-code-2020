use super::solution::{Error, Solution};
use std::collections::{HashMap, HashSet};

type Deck = Vec<usize>;

fn parse_deck(deck_text: &str) -> Deck {
    let mut parts = deck_text.lines();

    parts.next(); // Skip "Player..." line

    parts.map(|el| el.parse().unwrap()).collect()
}

fn parse_decks(cards_text: &str) -> (Deck, Deck) {
    let mut parts = cards_text.split("\n\n");

    (
        parse_deck(parts.next().unwrap()),
        parse_deck(parts.next().unwrap()),
    )
}

fn count_deck_score(deck: &Deck) -> usize {
    deck.iter()
        .rev()
        .enumerate()
        .map(|(i, &v)| (i + 1) * v)
        .sum()
}

fn simulate_combat_game<'a>(
    first_deck: &'a mut Deck,
    second_deck: &'a mut Deck,
) -> (&'a Deck, &'a Deck) {
    while !first_deck.is_empty() && !second_deck.is_empty() {
        let f = first_deck.remove(0);
        let s = second_deck.remove(0);

        if f > s {
            first_deck.push(f);
            first_deck.push(s);
        } else {
            second_deck.push(s);
            second_deck.push(f);
        }
    }

    (first_deck, second_deck)
}

fn generate_cache_key(first_deck: &Deck, second_deck: &Deck) -> String {
    let first_deck_key = first_deck
        .iter()
        .map(|c| c.to_string())
        .collect::<Vec<_>>()
        .join(",");
    let second_deck_key = second_deck
        .iter()
        .map(|c| c.to_string())
        .collect::<Vec<_>>()
        .join(",");

    format!("{}|{}", first_deck_key, second_deck_key)
}

fn simulate_recursive_combat_game<'a>(
    first_deck: &'a mut Deck,
    second_deck: &'a mut Deck,
    game_cache: &'a mut HashMap<String, bool>,
) -> (bool, &'a Deck, &'a Deck) {
    let game_cache_key = generate_cache_key(first_deck, second_deck);

    if game_cache.contains_key(&game_cache_key) {
        return (
            *game_cache.get(&game_cache_key).unwrap(),
            first_deck,
            second_deck,
        );
    }

    let mut round_cache: HashSet<String> = HashSet::new();
    while !first_deck.is_empty() && !second_deck.is_empty() {
        let round_cache_key = generate_cache_key(first_deck, second_deck);

        if round_cache.contains(&round_cache_key) {
            return (true, first_deck, second_deck);
        }

        round_cache.insert(round_cache_key);

        let f = first_deck.remove(0);
        let s = second_deck.remove(0);

        let mut res = f > s;

        if (f <= first_deck.len()) && (s <= second_deck.len()) {
            let mut first_sub_deck = (0..f).map(|i| first_deck[i]).collect::<Vec<usize>>();
            let mut second_sub_deck = (0..s).map(|i| second_deck[i]).collect::<Vec<usize>>();

            res = simulate_recursive_combat_game(
                &mut first_sub_deck,
                &mut second_sub_deck,
                game_cache,
            )
            .0;
        }

        if res {
            first_deck.push(f);
            first_deck.push(s);
        } else {
            second_deck.push(s);
            second_deck.push(f);
        }
    }

    let game_result = !first_deck.is_empty();
    game_cache.insert(game_cache_key, game_result);

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
    let mut game_cache = HashMap::new();
    let (res, fd, sd) = simulate_recursive_combat_game(first_deck, second_deck, &mut game_cache);

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
