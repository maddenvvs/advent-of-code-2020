import re
from typing import Callable, Iterable


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


def task_tests() -> None:
    test_passwords = [
        "1-3 a: abcde",
        "1-3 b: cdefg",
        "2-9 c: ccccccccc"
    ]

    assert count_old_valid_passwords(test_passwords) == 2
    assert count_current_valid_passwords(test_passwords) == 1


def run_tasks() -> None:
    with open("passwords.txt") as passwords_list:
        passwords = [password.strip() for password in passwords_list]

        print(f"Day 2-1: {count_old_valid_passwords(passwords)}")
        print(f"Day 2-2: {count_current_valid_passwords(passwords)}")


def main() -> None:
    task_tests()
    run_tasks()


if __name__ == "__main__":
    main()
