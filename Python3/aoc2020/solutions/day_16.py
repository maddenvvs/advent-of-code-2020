from __future__ import annotations

from dataclasses import dataclass
from typing import Callable, Dict, Iterator, List, Tuple

from .solution import Solution

FieldName = str
FieldValidator = Callable[[int], bool]
FieldRules = Dict[FieldName, FieldValidator]
Ticket = List[int]


class Graph:
    def __init__(self, n):
        self.g = [[] for _ in range(n)]

    def add_edge(self, u, v):
        self.g[u].append(v)
        self.g[v].append(u)

    def find_matching_using_Kuhn_alogrithm(self):
        match = [-1] * len(self.g)
        visited = set()
        has_augmented = True

        while has_augmented:
            has_augmented = False
            visited.clear()

            for u in range(len(self.g)):
                if match[u] == -1:
                    has_augmented = has_augmented or self.dfs(
                        u, visited, match)

        return match

    def dfs(self, u, visited, match):
        if u in visited:
            return False

        visited.add(u)

        for v in self.g[u]:
            if match[v] == -1 or self.dfs(match[v], visited, match):
                match[v] = u
                match[u] = v
                return True

        return False


@dataclass
class Notes:
    rules: FieldRules
    my_ticket: Ticket
    nearby_tickets: List[Ticket]

    def sum_of_invalid_values_in_nearby_tickets(self) -> int:
        return sum(self.identify_invalid_values_in_nearby_tickets())

    def identify_invalid_values_in_nearby_tickets(self) -> Iterator[int]:
        for nearby_ticket in self.nearby_tickets:
            for number in nearby_ticket:
                if self.is_value_not_valid_for_any_field(number):
                    yield number

    def is_value_not_valid_for_any_field(self, value: int) -> bool:
        return all(not validator(value) for validator in self.rules.values())

    def product_of_departure_values(self) -> int:
        product = 1
        for field, value in zip(self.find_field_order(), self.my_ticket):
            if field.startswith("departure"):
                product *= value

        return product

    def find_field_order(self) -> List[FieldName]:
        valid_tickets = self.find_valid_tickets(
            self.nearby_tickets + [self.my_ticket])

        rules_list = list(self.rules.items())
        fields_count = len(self.rules)
        graph = Graph(2 * fields_count)

        for column in range(fields_count):
            for rule_idx, (name, validator) in enumerate(rules_list, start=fields_count):
                for ticket in valid_tickets:
                    if not validator(ticket[column]):
                        break
                else:
                    graph.add_edge(column, rule_idx)

        matching = graph.find_matching_using_Kuhn_alogrithm()

        return [rules_list[matching[rule_idx]-fields_count][0] for rule_idx in range(fields_count)]

    def find_valid_tickets(self, tickets: List[Ticket]) -> List[Ticket]:
        return [t for t in tickets if self.is_ticket_valid(t)]

    def is_ticket_valid(self, ticket: Ticket) -> bool:
        return not any(self.is_value_not_valid_for_any_field(v) for v in ticket)

    @classmethod
    def from_str(cls, notes_text: str) -> Notes:
        parts = notes_text.split("\n\n")
        rules = cls.parse_rules(parts[0])
        my_ticket = cls.parse_my_ticket(parts[1])
        nearby_tickets = cls.parse_nearby_tickets(parts[2])

        return cls(rules, my_ticket, nearby_tickets)

    @staticmethod
    def parse_my_ticket(my_ticket_text: str) -> Ticket:
        return Notes.parse_ticket(my_ticket_text.lstrip("your ticket:\n"))

    @staticmethod
    def parse_nearby_tickets(nearby_tickets_text: str) -> List[Ticket]:
        tickets_str = nearby_tickets_text.lstrip("nearby tickets:\n")

        return [Notes.parse_ticket(t) for t in tickets_str.splitlines()]

    @staticmethod
    def parse_ticket(ticket_text: str) -> Ticket:
        return [int(i, base=10) for i in ticket_text.split(",")]

    @staticmethod
    def parse_rules(rules_text: str) -> FieldRules:
        rules = {}
        for rule in rules_text.splitlines():
            name, validator = Notes.parse_rule(rule)
            rules[name] = validator
        return rules

    @staticmethod
    def parse_rule(rule_text: str) -> Tuple[FieldName, FieldValidator]:
        name, values = rule_text.split(": ")
        return name, Notes.parse_value_intervals(values)

    @staticmethod
    def parse_value_intervals(value_intervals_text: str) -> FieldValidator:
        ranges = []
        for value_interval in value_intervals_text.split(" or "):
            from_text, to_text = value_interval.split("-")
            from_int = int(from_text, base=10)
            to_int = int(to_text, base=10)

            ranges.append((from_int, to_int))

        return Notes.create_field_validator(ranges)

    @staticmethod
    def create_field_validator(ranges: List[Tuple[int, int]]) -> FieldValidator:
        def validator(value: int) -> bool:
            return any(f <= value <= t for f, t in ranges)

        return validator


class Day16(Solution):

    def first_task(self, notes_text: str) -> str:
        notes = Notes.from_str(notes_text)

        return str(notes.sum_of_invalid_values_in_nearby_tickets())

    def second_task(self, notes_text: str) -> str:
        notes = Notes.from_str(notes_text)

        return str(notes.product_of_departure_values())
