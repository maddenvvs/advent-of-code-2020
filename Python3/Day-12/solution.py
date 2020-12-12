from __future__ import annotations

from dataclasses import dataclass
from enum import IntEnum
from typing import List

DIRECTIONS = (complex(1, 0), complex(0, 1), complex(-1, 0), complex(0, -1))


class Action(IntEnum):
    E = 0
    N = 1
    W = 2
    S = 3
    L = 4
    R = 5
    F = 6


@dataclass
class Instruction:
    action: Action
    value: int

    @classmethod
    def from_str(cls, instruction_text: str) -> Instruction:
        action_text, num_str = instruction_text[0], instruction_text[1:]

        return cls(Action[action_text], int(num_str, base=10))


def manhattan_distance(vector: complex) -> int:
    return abs(int(vector.real)) + abs(int(vector.imag))


def parse_instructions(instructions_text: str) -> List[Instruction]:
    return [Instruction.from_str(instr) for instr in instructions_text.splitlines()]


def run_simulation_with_waypoint(instructions: List[Instruction],
                                 *,
                                 waypoint: complex = complex(1, 0),
                                 move_waypoint: bool = False) -> complex:
    position = complex(0, 0)

    for instruction in instructions:
        if instruction.action == Action.F:
            position += waypoint * instruction.value
        elif instruction.action == Action.R:
            waypoint *= pow(DIRECTIONS[Action.S], instruction.value // 90)
        elif instruction.action == Action.L:
            waypoint *= pow(DIRECTIONS[Action.N], instruction.value // 90)
        else:
            direction = DIRECTIONS[instruction.action]

            if move_waypoint:
                waypoint += direction * instruction.value
            else:
                position += direction * instruction.value

    return position


def simulate_instructions_with_rotation(instructions: List[Instruction]) -> int:
    position = run_simulation_with_waypoint(instructions)

    return manhattan_distance(position)


def simulate_instructions_with_waypoint(instructions: List[Instruction]) -> int:
    position = run_simulation_with_waypoint(
        instructions, waypoint=complex(10, 1), move_waypoint=True)

    return manhattan_distance(position)


def test_tasks() -> None:
    test_instructions_1_text = """F10
N3
F7
R90
F11"""

    test_instructions_1 = parse_instructions(test_instructions_1_text)
    assert simulate_instructions_with_rotation(test_instructions_1) == 25
    assert simulate_instructions_with_waypoint(test_instructions_1) == 286


def run_tasks() -> None:
    with open("instructions.txt") as instruction_file:
        instructions = parse_instructions(instruction_file.read())

        print(f"Day 12-1: {simulate_instructions_with_rotation(instructions)}")
        print(f"Day 12-2: {simulate_instructions_with_waypoint(instructions)}")


def main() -> None:
    test_tasks()
    run_tasks()


if __name__ == "__main__":
    main()
