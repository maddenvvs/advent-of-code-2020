from __future__ import annotations

from collections import defaultdict
from typing import Dict, List, Tuple

from .solution import Solution

BagColor = str


SHINY_GOLD = "shiny gold"


class BagsRules:
    __slots__ = "_graph"

    def __init__(self):
        self._graph: Dict[BagColor,
                          List[Tuple[BagColor, int]]] = defaultdict(list)

    def _add_edge(self, u: BagColor, v: BagColor, c: int) -> None:
        self._graph[u].append((v, c))

    def _reverse(self) -> BagsRules:
        reversed_graph = BagsRules()

        for u, edges in self._graph.items():
            for v, c in edges:
                reversed_graph._add_edge(v, u, c)

        return reversed_graph

    def _count_nodes_reachable_from(self, bag: BagColor) -> int:
        stack = [bag]
        visited = set(stack)
        count = 0

        while stack:
            curr = stack.pop()
            count += 1

            for v, _ in self._graph[curr]:
                if v not in visited:
                    visited.add(v)
                    stack.append(v)

        return count

    def count_bag_colors_containing(self, bag: BagColor) -> int:
        return self._reverse()._count_nodes_reachable_from(bag) - 1

    def count_bags_inside(self, bag: BagColor) -> int:
        total = 0

        for other, count in self._graph[bag]:
            total += count * (self.count_bags_inside(other) + 1)

        return total

    @ classmethod
    def from_rules_text(cls, rules_text: str) -> BagsRules:
        bags_rules = cls()

        for bag_rule_text in rules_text.splitlines():
            container_bag, bags_inside = cls._parse_bag_rule(bag_rule_text)
            for bag_inside, count in bags_inside:
                bags_rules._add_edge(container_bag, bag_inside, count)

        return bags_rules

    @ staticmethod
    def _parse_bag_rule(bag_rule: str) -> Tuple[str, List[Tuple[str, int]]]:
        words = bag_rule.split()
        container_bag = " ".join(words[:2])
        bags_inside: List[Tuple[str, int]] = []

        if words[4] == "no":
            return container_bag, bags_inside

        for i in range(4, len(words), 4):
            count_str, *bag_color = words[i:i+3]
            count = int(count_str, base=10)
            other_bag = " ".join(bag_color)
            bags_inside.append((other_bag, count))

        return container_bag, bags_inside


class Day07(Solution):

    def first_task(self, bags_text: str) -> str:
        bags_rules = BagsRules.from_rules_text(bags_text)

        return str(bags_rules.count_bag_colors_containing(SHINY_GOLD))

    def second_task(self, bags_text: str) -> str:
        bags_rules = BagsRules.from_rules_text(bags_text)

        return str(bags_rules.count_bags_inside(SHINY_GOLD))
