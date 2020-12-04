from string import hexdigits, digits
from typing import Callable, Dict, List

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


def test_tasks() -> None:
    test_passwords_file = """ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in"""

    test_strong_invalid_passwords = """eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007"""

    test_strong_valid_passwords = """pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"""

    assert count_passwords_with_weak_validation(test_passwords_file) == 2
    assert count_passwords_with_strong_validation(
        test_strong_invalid_passwords) == 0
    assert count_passwords_with_strong_validation(
        test_strong_valid_passwords) == 4


def run_tasks() -> None:
    with open("passports.txt") as passports_file:
        password_file_content = passports_file.read()

        print(
            f"Day 4-1: {count_passwords_with_weak_validation(password_file_content)}")
        print(
            f"Day 4-2: {count_passwords_with_strong_validation(password_file_content)}")


def main() -> None:
    test_tasks()
    run_tasks()


if __name__ == "__main__":
    main()
