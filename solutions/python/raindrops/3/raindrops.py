#!/usr/bin/env python3

FACTOR_TO_VOWEL = ((3, "i"), (5, "a"), (7, "o"))


def raindrops(number):
    vowels = [vowel for factor, vowel in FACTOR_TO_VOWEL if number % factor == 0]

    if vowels:
        return "".join(f"Pl{v}ng" for v in vowels)
    else:
        return str(number)
