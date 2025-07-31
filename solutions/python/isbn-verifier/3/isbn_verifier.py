#!/usr/bin/env python3

from typing import Optional


def isbn_character_to_int(c: str) -> Optional[int]:
    if c.isdecimal():
        return int(c)
    elif c == "X":
        return 10
    else:
        return None


def verify(isbn: str) -> bool:
    digits = [d for d in [isbn_character_to_int(c) for c in isbn] if d is not None]

    if len(digits) != 10:
        return False

    if any(d == 10 for d in digits[:-1]):
        return False

    return sum((d * i for d, i in zip(digits, range(10, 0, -1)))) % 11 == 0
