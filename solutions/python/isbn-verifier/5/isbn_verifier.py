#!/usr/bin/env python3

"""
Check if strings represent valid ISBN numbers.
"""

from typing import Optional


def _isbn_character_to_int(character: str) -> Optional[int]:
    if character.isdecimal():
        return int(character)

    if character == "X":
        return 10

    return None


def is_valid(isbn: str) -> bool:
    """
    Return True if the string isbn represents a valid ISBN number.
    """

    digits = [d for d in (_isbn_character_to_int(c) for c in isbn) if d is not None]

    if len(digits) != 10:
        return False

    if 10 in digits[:-1]:
        return False

    return sum(d * i for d, i in enumerate(reversed(digits), 1)) % 11 == 0
