from string import hexdigits, digits
from typing import Callable, Dict, List

from .solution import Solution

Passport = Dict[str, str]
Validator = Callable[[str], bool]


def is_number(num: str, length: int) -> bool:
    return len(num) == length and all(ch in digits for ch in num)


def is_number_between(num: str, length: int, _from: int, _to: int) -> bool:
    return is_number(num, length) and _from <= int(num, base=10) <= _to


def is_year_between(_from: int, _to: int) -> Validator:
    def _validator(num: str):
        return is_number_between(num, 4, _from, _to)

    return _validator


def is_height_valid_cm(num: str) -> bool:
    return is_number_between(num, 3, 150, 193)


def is_height_valid_in(num: str) -> bool:
    return is_number_between(num, 2, 59, 76)


def is_height_valid(value: str) -> bool:
    suffix = value[-2:]
    if suffix == "cm":
        return is_height_valid_cm(value[:-2])

    if suffix == "in":
        return is_height_valid_in(value[:-2])

    return False


def is_hair_color_valid(value: str) -> bool:
    return len(value) == 7 and value[0] == "#" and all(ch in hexdigits for ch in value[1:])


def is_eye_color_valid(value: str) -> bool:
    return value in ("amb", "blu", "brn", "gry", "grn", "hzl", "oth")


def is_pid_valid(value: str) -> bool:
    return is_number(value, 9)


FIELD_VALIDATORS: Dict[str, Validator] = {
    "byr": is_year_between(1920, 2002),
    "iyr": is_year_between(2010, 2020),
    "eyr": is_year_between(2020, 2030),
    "hgt": is_height_valid,
    "hcl": is_hair_color_valid,
    "ecl": is_eye_color_valid,
    "pid": is_pid_valid,
}


def is_passport_valid_weak(passport: Passport) -> bool:
    return len(passport) == 8 or (len(passport) == 7 and "cid" not in passport)


def is_passport_valid_strong(passport: Passport) -> bool:
    if not is_passport_valid_weak(passport):
        return False

    return all(validator(passport[field]) for field, validator in FIELD_VALIDATORS.items())


def parse_password(password_text: str) -> Passport:
    password_dict = {}

    for component in password_text.split():
        name, value = component.split(":")
        password_dict[name] = value

    return password_dict


def parse_passwords(passwords_text: str) -> List[Passport]:
    passwords_parts = (line.strip() for line in passwords_text.split("\n\n"))

    return [parse_password(password) for password in passwords_parts]


def count_valid_passports(passwords_text: str, password_policy: Callable[[Passport], bool]) -> int:
    passports = parse_passwords(passwords_text)
    return sum(password_policy(passport) for passport in passports)


def count_passwords_with_weak_validation(passwords_text: str) -> int:
    return count_valid_passports(passwords_text, is_passport_valid_weak)


def count_passwords_with_strong_validation(passwords_text: str) -> int:
    return count_valid_passports(passwords_text, is_passport_valid_strong)


class Day04(Solution):

    def first_task(self, passports_text: str) -> str:
        return str(count_passwords_with_weak_validation(passports_text))

    def second_task(self, passports_text: str) -> str:
        return str(count_passwords_with_strong_validation(passports_text))
