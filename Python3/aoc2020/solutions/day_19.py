from __future__ import annotations

from abc import ABC, abstractmethod
from typing import Iterable, Iterator, List, Tuple

from .solution import Solution

RuleName = int


class Rule(ABC):

    @abstractmethod
    def try_match(self, message: str, idx: int) -> Iterator[int]:
        pass


class MatchLetter(Rule):
    __slots__ = "letter"

    def __init__(self, letter: str):
        self.letter = letter

    def try_match(self, message: str, idx: int) -> Iterator[int]:
        if idx < len(message) and message[idx] == self.letter:
            yield idx + 1


class MatchRuleName(Rule):
    __slots__ = "rules", "name"

    rules: Rules

    def __init__(self, rules: Rules, name: RuleName):
        self.rules = rules
        self.name = name

    def try_match(self, message: str, idx: int) -> Iterator[int]:
        yield from self.rules.graph[self.name].try_match(message, idx)


class And(Rule):
    __slots__ = "rules"

    def __init__(self, rules: Iterable[Rule]):
        self.rules = rules

    def try_match(self, message: str, idx: int) -> Iterator[int]:
        next_idx = [idx]
        for rule in self.rules:
            new_idx = []
            for i in next_idx:
                new_idx += list(rule.try_match(message, i))
            next_idx = new_idx

        yield from next_idx


class Or(Rule):
    __slots__ = "rules"

    def __init__(self, rules: Iterable[Rule]):
        self.rules = rules

    def try_match(self, message: str, idx: int) -> Iterator[int]:
        for rule in self.rules:
            yield from rule.try_match(message, idx)


class Rules:
    __slots__ = "graph"

    def __init__(self):
        self.graph = {}

    def add_rule(self, name: RuleName, definition: Rule) -> None:
        self.graph[name] = definition

    def matches_rule(self, message: str, rule_name: RuleName) -> bool:
        last_idx_match = list(self.graph[rule_name].try_match(message, 0))

        return len(message) in last_idx_match

    def make_changes_in_rules(self):
        changes = ("8: 42 | 42 8\n"
                   "11: 42 31 | 42 11 31")

        self.make_rules_change(changes)

    def make_rules_change(self, new_rules: str) -> None:
        Rules.populate_rules(self, new_rules)

    @ classmethod
    def from_str(cls, rules_text: str) -> Rules:
        rules = cls()
        cls.populate_rules(rules, rules_text)

        return rules

    @classmethod
    def populate_rules(cls, rules: Rules, rules_text: str) -> None:
        for rule_text in rules_text.splitlines():
            rules.add_rule(*cls.parse_rule(rule_text, rules))

    @ classmethod
    def parse_rule(cls, rule_text: str, rules: Rules) -> Tuple[RuleName, Rule]:
        name_str, def_str = rule_text.split(": ")
        name = int(name_str, base=10)

        alternatives = def_str.split(" | ")
        if len(alternatives) > 1:
            return name, Or([cls.parse_rule_definition(alt, rules) for alt in alternatives])
        else:
            return name, cls.parse_rule_definition(alternatives[0], rules)

    @ classmethod
    def parse_rule_definition(cls, definition_text: str, rules: Rules) -> Rule:
        parts = definition_text.split()
        if len(parts) > 1:
            return And([MatchRuleName(rules, int(r, base=10)) for r in parts])
        else:
            if parts[0].startswith("\""):
                return MatchLetter(parts[0][1])

            return MatchRuleName(rules, int(parts[0], base=10))


def parse_input_messages(messages_text: str) -> Tuple[Rules, List[str]]:
    rules_text, messages = messages_text.split("\n\n")

    return Rules.from_str(rules_text), messages.splitlines()


def count_messages_match_rule_0(rules: Rules, messages: Iterable[str]) -> int:
    return sum(rules.matches_rule(message, 0) for message in messages)


class Day19(Solution):

    def first_task(self, messages_text: str) -> str:
        rules, messages = parse_input_messages(messages_text)

        return str(count_messages_match_rule_0(rules, messages))

    def second_task(self, messages_text: str) -> str:
        rules, messages = parse_input_messages(messages_text)
        rules.make_changes_in_rules()

        return str(count_messages_match_rule_0(rules, messages))
