use super::solution::{Error, Solution};
use std::collections::{HashMap, HashSet};

struct Food<'a> {
    ingredients: Vec<&'a str>,
    allergens: Vec<&'a str>,
}

impl Food<'_> {
    fn new(s: &str) -> Food {
        let mut parts = s.split(" (contains ");
        let ingredients_text = parts.next().unwrap();
        let allergens_text = parts.next().unwrap().trim_end_matches(')');

        Food {
            ingredients: ingredients_text.split_whitespace().collect(),
            allergens: allergens_text.split(", ").collect(),
        }
    }
}

struct Graph {
    g: HashMap<String, Vec<String>>,
}

impl Graph {
    fn new() -> Graph {
        Graph { g: HashMap::new() }
    }

    fn add_edge(&mut self, u: &str, v: &str) {
        self.g
            .entry(u.to_string())
            .or_insert(Vec::new())
            .push(v.to_string());
        self.g
            .entry(v.to_string())
            .or_insert(Vec::new())
            .push(u.to_string());
    }

    fn find_matching_using_kuhn_algorithm(&self) -> HashMap<String, String> {
        let mut matching: HashMap<&String, &String> = HashMap::new();
        let mut visited: HashSet<&String> = HashSet::new();
        let mut has_augmented = true;

        while has_augmented {
            has_augmented = false;
            visited.clear();

            for u in self.g.keys() {
                if !matching.contains_key(u) && !visited.contains(u) {
                    has_augmented |= self.dfs(u, &mut visited, &mut matching);
                }
            }
        }

        matching
            .iter()
            .map(|(&l, &r)| (l.to_string(), r.to_string()))
            .collect()
    }

    fn dfs<'a>(
        &'a self,
        u: &'a String,
        visited: &mut HashSet<&'a String>,
        matching: &mut HashMap<&'a String, &'a String>,
    ) -> bool {
        if visited.contains(u) {
            return false;
        }

        visited.insert(u);

        for v in self.g.get(u).unwrap().iter() {
            if !matching.contains_key(v) || self.dfs(matching.get(v).unwrap(), visited, matching) {
                matching.insert(v, u);
                matching.insert(u, v);
                return true;
            }
        }

        false
    }
}

fn parse_food_list(foods_text: &str) -> Vec<Food> {
    foods_text.lines().map(Food::new).collect()
}

fn find_allergen_candidates<'a>(food_list: &[Food<'a>]) -> HashMap<&'a str, HashSet<&'a str>> {
    let all_ingredients: HashSet<&str> = food_list
        .iter()
        .flat_map(|f| &f.ingredients)
        .copied()
        .collect();

    let mut allergen_candidates: HashMap<&str, HashSet<&str>> = HashMap::new();
    for food in food_list.iter() {
        let food_ingredients: HashSet<&str> = food.ingredients.iter().copied().collect();

        for allergen in food.allergens.iter() {
            let val = allergen_candidates
                .entry(allergen)
                .or_insert_with(HashSet::new);

            if val.is_empty() {
                *val = all_ingredients
                    .intersection(&food_ingredients)
                    .copied()
                    .collect();
            } else {
                *val = val.intersection(&food_ingredients).copied().collect();
            }
        }
    }

    allergen_candidates
}

fn count_allergen_free_ingredients(food_list: &[Food]) -> usize {
    let possible_allergens: HashSet<&str> = find_allergen_candidates(food_list)
        .values()
        .fold(HashSet::new(), |acc, el| acc.union(el).copied().collect());

    food_list
        .iter()
        .flat_map(|f| &f.ingredients)
        .filter(|&i| !possible_allergens.contains(i))
        .count()
}

fn find_allergen_list(food_list: &[Food]) -> String {
    let allergen_candidates = find_allergen_candidates(food_list);
    let mut graph = Graph::new();

    for (allergen, candidates) in &allergen_candidates {
        for candidate in candidates {
            graph.add_edge(allergen, candidate)
        }
    }

    let matching = graph.find_matching_using_kuhn_algorithm();
    let mut foreign_allergens: Vec<&str> = allergen_candidates.keys().copied().collect();
    foreign_allergens.sort_unstable();

    foreign_allergens
        .iter()
        .map(|&el| matching.get(el).unwrap().as_str())
        .collect::<Vec<&str>>()
        .join(",")
}

pub struct Day21 {}

impl Solution for Day21 {
    fn first_task(&self, foods_text: &str) -> Result<String, Error> {
        let food_list = parse_food_list(&foods_text);

        Ok(count_allergen_free_ingredients(&food_list).to_string())
    }

    fn second_task(&self, foods_text: &str) -> Result<String, Error> {
        let food_list = parse_food_list(&foods_text);

        Ok(find_allergen_list(&food_list))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_task() {
        let test_ingredients_text = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";

        let test_ingredients = parse_food_list(&test_ingredients_text);

        assert_eq!(count_allergen_free_ingredients(&test_ingredients), 5);
    }

    #[test]
    fn test_second_task() {
        let test_ingredients_text = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";

        let test_ingredients = parse_food_list(&test_ingredients_text);

        assert_eq!(
            find_allergen_list(&test_ingredients),
            String::from("mxmxvkd,sqjhc,fvjkl")
        );
    }
}
