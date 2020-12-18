from typing import Union, Iterable, List

from .solution import Solution


def parse_expressions(expressions_text: str) -> List[str]:
    return expressions_text.splitlines()


def eval_op(f, op, s):
    if op == "+":
        return f + s

    return f * s


def evaluate_expression(expression: str) -> int:
    stack: List[Union[int, str]] = []

    def try_swallow(value):
        if stack and stack[-1] in "+*":
            op = stack.pop()
            first = stack.pop()
            stack.append(eval_op(first, op, value))
        else:
            stack.append(value)

    for ch in expression:
        if ch == ")":
            res = stack.pop()
            stack.pop()  # Throw away "("
            try_swallow(res)

        elif ch.isdigit():
            d = int(ch, base=10)
            try_swallow(d)

        elif not ch.isspace():
            stack.append(ch)

    return int(stack[0])


def evaluate_expression_advanced(expression: str) -> int:
    stack: List[Union[int, str]] = []

    def try_swallow(values):
        while len(stack) > 1 and isinstance(stack[-2], str) and stack[-2] in values:
            s = stack.pop()
            op = stack.pop()
            f = stack.pop()
            stack.append(eval_op(s, op, f))

    def simplify():
        try_swallow("+*")

    for ch in expression:
        if ch.isspace():
            continue

        if ch == "(":
            stack.append(ch)

        elif ch == ")":
            simplify()

            res = stack.pop()
            stack.pop()  # Throw away "("
            stack.append(res)

        elif ch.isdigit():
            d = int(ch, base=10)
            stack.append(d)

        elif ch == "*":
            simplify()
            stack.append(ch)

        elif ch == "+":
            try_swallow("+")
            stack.append(ch)

    simplify()

    return int(stack[0])


def sum_of_expressions(expressions: Iterable[str]) -> int:
    return sum(evaluate_expression(exp) for exp in expressions)


def sum_of_expressions_advanced(expressions: Iterable[str]) -> int:
    return sum(evaluate_expression_advanced(exp) for exp in expressions)


class Day18(Solution):

    def first_task(self, expressions_text: str) -> str:
        expressions = parse_expressions(expressions_text)

        return str(sum_of_expressions(expressions))

    def second_task(self, expressions_text: str) -> str:
        expressions = parse_expressions(expressions_text)

        return str(sum_of_expressions_advanced(expressions))
