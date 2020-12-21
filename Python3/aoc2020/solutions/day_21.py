from __future__ import annotations

from collections import defaultdict
from dataclasses import dataclass
from typing import Dict, Generic, List, Set, TypeVar

from .solution import Solution

T = TypeVar("T")
GraphMatching = Dict[T, T]


class Graph(Generic[T]):
    __slots__ = "g"

    g: Dict[T, List[T]]

    def __init__(self) -> None:
        self.g = defaultdict(list)

    def add_edge(self, u: T, v: T) -> None:
        self.g[u].append(v)
        self.g[v].append(u)

    def find_matching_using_Kuhn_alogrithm(self) -> GraphMatching:
        match: GraphMatching = {}
        visited: Set[T] = set()
        has_augmented = True

        while has_augmented:
            has_augmented = False
            visited.clear()

            for u in self.g:
                if u not in match:
                    has_augmented = has_augmented or self.dfs(
                        u, visited, match)

        return match

    def dfs(self, u: T, visited: Set[T], match: GraphMatching) -> bool:
        if u in visited:
            return False

        visited.add(u)
        for v in self.g[u]:
            if v not in match or self.dfs(match[v], visited, match):
                match[v] = u
                match[u] = v
                return True

        return False


@dataclass
class Food:
    ingredients: List[str]
    allergens: List[str]

    @classmethod
    def from_str(cls, food_text: str) -> Food:
        ingredients_text, allergens_text = food_text.split(" (contains ")
        allergens_text = allergens_text[:-1]

        ingredients = ingredients_text.split()
        allergens = allergens_text.split(", ")

        return cls(ingredients, allergens)


def parse_food_list(foods_text: str) -> List[Food]:
    return [Food.from_str(f) for f in foods_text.splitlines()]


def find_allergen_candidates(food_list: List[Food]) -> Dict[str, Set[str]]:
    all_ingredients = set()
    for food in food_list:
        for ingredient in food.ingredients:
            all_ingredients.add(ingredient)

    allergen_candidates: Dict[str, Set[str]
                              ] = defaultdict(lambda: all_ingredients)
    for food in food_list:
        for allergen in food.allergens:
            allergen_candidates[allergen] = allergen_candidates[allergen] & set(
                food.ingredients)

    return allergen_candidates


def count_allergen_free_ingredients(food_list: List[Food]) -> int:
    allergen_candidates = find_allergen_candidates(food_list)

    possible_allergens = set()
    for allergen_candidate in allergen_candidates.values():
        possible_allergens |= allergen_candidate

    return sum(ing not in possible_allergens for f in food_list for ing in f.ingredients)


def find_allergen_list(food_list: List[Food]) -> str:
    allergen_candidates = find_allergen_candidates(food_list)
    graph = Graph[str]()

    for allergen, candidates in allergen_candidates.items():
        for candidate in candidates:
            graph.add_edge(allergen, candidate)

    matching = graph.find_matching_using_Kuhn_alogrithm()
    foreign_allergens = [matching[a]
                         for a in sorted(allergen_candidates.keys())]

    return ",".join(foreign_allergens)


class Day21(Solution):

    def first_task(self, foods_text: str) -> str:
        food_list = parse_food_list(foods_text)

        return str(count_allergen_free_ingredients(food_list))

    def second_task(self, foods_text: str) -> str:
        food_list = parse_food_list(foods_text)

        return find_allergen_list(food_list)
