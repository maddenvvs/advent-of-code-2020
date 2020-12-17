from __future__ import annotations

from abc import ABC
from collections import defaultdict
from typing import Dict, List, Tuple, Optional

from .solution import Solution

ADDRESS_SIZE = 36


class Command(ABC):
    pass


class Mask(Command):
    __slots__ = "value"

    INITIAL_MASK = "X" * ADDRESS_SIZE

    def __init__(self, value: str):
        self.value = value


class MemorySet(Command):
    __slots__ = "address", "value"

    def __init__(self, address: str, value: int):
        self.address = address
        self.value = value
        self.address_val = int(address, base=10)
        self.bin_address_val = bin(self.address_val)[2:].zfill(ADDRESS_SIZE)


class TrieNode:
    __slots__ = "left", "right", "value"

    left: Optional[TrieNode]
    right: Optional[TrieNode]
    value: int

    def __init__(self, value: int = 0, left: Optional[TrieNode] = None, right: Optional[TrieNode] = None):
        self.value = value
        self.left = left
        self.right = right

    def clone(self) -> TrieNode:
        if self.left and (self.left is self.right):
            cloned_node = cloned_node = self.left.clone()

            return TrieNode(self.value, cloned_node, cloned_node)

        left_copy, right_copy = None, None

        if self.left:
            left_copy = self.left.clone()
        if self.right:
            right_copy = self.right.clone()

        return TrieNode(self.value, left_copy, right_copy)

    def add_value_at(self, address: str, value: int, *, position: int = 0):
        if position >= len(address):
            self.value = value

            return

        addr_bit = address[position]
        if addr_bit == "1":
            if not self.right:
                self.right = TrieNode()
            elif self.left is self.right:
                self.right = self.left.clone()

            self.right.add_value_at(address, value, position=position + 1)
        elif addr_bit == "0":
            if not self.left:
                self.left = TrieNode()
            elif self.left is self.right:
                self.left = self.right.clone()

            self.left.add_value_at(address, value, position=position + 1)
        elif addr_bit == "X":
            node = TrieNode()
            if not self.left:
                self.left = node
            if not self.right:
                self.right = node

            self.left.add_value_at(address, value, position=position + 1)
            if self.left is not self.right:
                self.right.add_value_at(address, value, position=position + 1)

        self.value = ((self.left and self.left.value) or 0) + \
            ((self.right and self.right.value) or 0)


def parse_command(command_text: str) -> Command:
    parts = command_text.split(" = ")
    if parts[0] == "mask":
        return Mask(parts[1])

    return MemorySet(parts[0][4:-1], int(parts[1], base=10))


def parse_program(program_text: str) -> List[Command]:
    return [parse_command(c) for c in program_text.splitlines()]


def emulate_program_v1(program: List[Command]) -> int:
    memory: Dict[int, int] = defaultdict(int)
    mask = Mask.INITIAL_MASK

    for command in program:
        if isinstance(command, Mask):
            mask = command.value
        elif isinstance(command, MemorySet):
            value = command.value
            for i, v in enumerate(reversed(mask)):
                if v == "0":
                    value &= ~(1 << i)
                elif v == "1":
                    value |= (1 << i)
            memory[command.address_val] = value

    return sum(memory.values())


def emulate_program_v2(program: List[Command]) -> int:
    memory = TrieNode()
    mask = Mask.INITIAL_MASK

    def apply_mask(val: str) -> str:
        res = []

        for m, v in zip(reversed(mask), reversed(val)):
            if m == "X":
                res.append(m)
            elif m == "1":
                res.append("1")
            elif m == "0":
                res.append(v)

        return "".join(reversed(res))

    for command in program:
        if isinstance(command, Mask):
            mask = command.value
        elif isinstance(command, MemorySet):
            address = apply_mask(command.bin_address_val)
            value = command.value
            memory.add_value_at(address, value)

    return memory.value


def find_memory_values_sum_v1(program: List[Command]) -> int:
    return emulate_program_v1(program)


def find_memory_values_sum_v2(program: List[Command]) -> int:
    return emulate_program_v2(program)


class Day14(Solution):

    def first_task(self, program_text: str) -> str:
        program = parse_program(program_text)

        return str(find_memory_values_sum_v1(program))

    def second_task(self, program_text: str) -> str:
        program = parse_program(program_text)

        return str(find_memory_values_sum_v2(program))
