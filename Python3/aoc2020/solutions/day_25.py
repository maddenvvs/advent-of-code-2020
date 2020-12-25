from typing import Tuple

from .solution import Solution

MODULO = 20201227


def parse_public_keys(keys_text: str) -> Tuple[int, int]:
    first, second = keys_text.splitlines()

    return int(first, base=10), int(second, base=10)


def find_loop_size(public_key: int) -> int:
    subject_number = 7
    loop_size = 1

    while subject_number != public_key:
        subject_number = (subject_number * 7) % MODULO
        loop_size += 1

    return loop_size


def find_encryption_key(door_public_key: int, key_public_key: int) -> int:
    loop = find_loop_size(door_public_key)

    enc_key = key_public_key
    for _ in range(loop-1):
        enc_key = (enc_key * key_public_key) % MODULO

    return enc_key


class Day25(Solution):

    def first_task(self, keys_text: str) -> str:
        first_key, second_key = parse_public_keys(keys_text)

        return str(find_encryption_key(first_key, second_key))

    def second_task(self, keys_text: str) -> str:
        return "I did it!!!"
