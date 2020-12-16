from __future__ import annotations

from dataclasses import dataclass
from typing import Callable, Dict, Iterator, List, Tuple

FieldName = str
FieldValidator = Callable[[int], bool]
FieldRules = Dict[FieldName, FieldValidator]
Ticket = List[int]


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

        possible_fields: List[List[FieldName]] = []
        for column in range(len(self.my_ticket)):
            possible_fields.append([])

            for name, validator in self.rules.items():
                for ticket in valid_tickets:
                    if not validator(ticket[column]):
                        break
                else:
                    possible_fields[column].append(name)

        def backtrack(column: int, temp_order: List[FieldName]) -> List[FieldName]:
            if column >= len(possible_fields):
                return temp_order
            else:
                for name in possible_fields[column]:
                    if name in temp_order:
                        continue

                    temp_order.append(name)
                    res = backtrack(column + 1, temp_order)
                    if res:
                        return res
                    temp_order.pop()

                return []

        return backtrack(0, [])

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


def test_tasks() -> None:
    test_notes_1_txt = """class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12"""

    test_notes_1 = Notes.from_str(test_notes_1_txt)
    assert test_notes_1.sum_of_invalid_values_in_nearby_tickets() == 71

    test_notes_2_txt = """class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9"""

    test_notes_2 = Notes.from_str(test_notes_2_txt)
    assert test_notes_2.find_field_order() == ["row", "class", "seat"]


def run_tasks() -> None:
    with open("notes.txt") as notes_file:
        notes = Notes.from_str(notes_file.read())

        print(f"Day 16-1: {notes.sum_of_invalid_values_in_nearby_tickets()}")
        print(f"Day 16-2: {notes.product_of_departure_values()}")


def main() -> None:
    test_tasks()
    run_tasks()


if __name__ == "__main__":
    main()
