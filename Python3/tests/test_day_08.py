from aoc2020.solutions.day_08 import parse_program, \
    find_acc_value_before_entering_loop, \
    find_acc_value_in_correct_program


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
