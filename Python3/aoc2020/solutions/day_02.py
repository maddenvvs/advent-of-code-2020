import re
from typing import Callable, Iterable

from .solution import Solution

PASSWORD_REGEX_PATTERN = re.compile(r"^(\d+)-(\d+) (\w): (\w+)$")


def current_policy(first_number: int, second_number: int, letter: str, password: str) -> bool:
    return (password[first_number - 1] == letter) ^ (password[second_number - 1] == letter)


def old_policy(first_number: int, second_number: int, letter: str, password: str) -> bool:
    occurences_of_letter = sum(ch == letter for ch in password)

    return first_number <= occurences_of_letter <= second_number


def is_password_valid(password: str, password_policy: Callable[[int, int, str, str], bool]) -> bool:
    password_text_match = PASSWORD_REGEX_PATTERN.fullmatch(password)

    if password_text_match is None:
        return False

    first_number_str, second_number_str, letter, password_text = password_text_match.groups()
    first_number = int(first_number_str, base=10)
    second_number = int(second_number_str, base=10)

    return password_policy(first_number, second_number, letter, password_text)


def count_old_valid_passwords(passwords: Iterable[str]) -> int:
    return sum(is_password_valid(password, old_policy) for password in passwords)


def count_current_valid_passwords(passwords: Iterable[str]) -> int:
    return sum(is_password_valid(password, current_policy) for password in passwords)


class Day02(Solution):

    def first_task(self, passwords_text: str) -> str:
        passwords = [password.strip()
                     for password in passwords_text.splitlines()]

        return str(count_old_valid_passwords(passwords))

    def second_task(self, passwords_text: str) -> str:
        passwords = [password.strip()
                     for password in passwords_text.splitlines()]

        return str(count_current_valid_passwords(passwords))
