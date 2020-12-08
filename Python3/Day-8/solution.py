from __future__ import annotations

from dataclasses import dataclass
from typing import Generator, List, Optional, Sequence, Tuple


@dataclass
class Command:
    op_code: str
    argument: int

    @property
    def is_jmp(self):
        return self.op_code == "jmp"

    @property
    def is_nop(self):
        return self.op_code == "nop"

    @property
    def is_acc(self):
        return self.op_code == "acc"

    @classmethod
    def from_str(cls, command: str) -> Command:
        op_code, arg = command.split()
        return cls(op_code, int(arg, base=10))

    def try_fix(self):
        if self.is_nop:
            self.op_code = "jmp"
        elif self.is_jmp:
            self.op_code = "nop"


@dataclass
class Registers:
    acc: int
    ip: int


def parse_program(program: str) -> List[Command]:
    return [Command.from_str(c) for c in program.splitlines()]


def emulate(commands: Sequence[Command]) -> Generator[Registers, None, None]:
    registers = Registers(0, 0)

    yield registers

    while registers.ip < len(commands):
        command = commands[registers.ip]
        if command.is_nop:
            registers.ip += 1
        elif command.is_jmp:
            registers.ip += command.argument
        elif command.is_acc:
            registers.acc += command.argument
            registers.ip += 1

        yield registers


def loop_protected_emulate(lines: Sequence[Command]) -> Tuple[bool, Registers]:
    visited_ips = set()

    for registers in emulate(lines):
        if registers.ip in visited_ips:
            return True, registers
        visited_ips.add(registers.ip)

    return False, registers


def find_acc_value_before_entering_loop(lines: Sequence[Command]) -> int:
    has_loop, registers = loop_protected_emulate(lines)

    assert has_loop, "Initial program has to have loop"

    return registers.acc


def find_acc_value_in_correct_program(lines: Sequence[Command]) -> Optional[int]:
    for command in lines:
        command.try_fix()

        has_loop, registers = loop_protected_emulate(lines)
        if registers.ip == len(lines):
            return registers.acc

        command.try_fix()

    return None


def test_tasks() -> None:
    test_program_text = """nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"""

    program = parse_program(test_program_text)

    assert find_acc_value_before_entering_loop(program) == 5
    assert find_acc_value_in_correct_program(program) == 8


def run_tasks() -> None:
    with open("program.txt") as program_file:
        program = parse_program(program_file.read())

        print(f"Day 8-1: {find_acc_value_before_entering_loop(program)}")
        print(f"Day 8-2: {find_acc_value_in_correct_program(program)}")


def main() -> None:
    test_tasks()
    run_tasks()


if __name__ == "__main__":
    main()
