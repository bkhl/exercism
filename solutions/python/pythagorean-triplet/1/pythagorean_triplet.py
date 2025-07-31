from functools import reduce
from math import gcd


def primitive_triplets(number_in_triplet):
    if number_in_triplet % 2 != 0:
        raise ValueError("odd numbers not valid")

    triplets = set()

    for n in range(0, number_in_triplet):
        for m in range(n + 1, number_in_triplet):
            if m - n < 1 or (m - n) % 2 == 0:
                continue

            a = m ** 2 - n ** 2
            b = 2 * m * n
            c = m ** 2 + n ** 2

            if number_in_triplet in (a, b, c) and reduce(gcd, (a, b, c)) == 1:
                triplets.add(tuple(sorted((a, b, c))))

    return triplets


def triplets_in_range(range_start, range_end):
    triplets = set()

    for a in range(range_start, range_end + 1):
        for b in range(a, range_end + 1):
            for c in range(a, range_end + 1):
                if a ** 2 + b ** 2 == c ** 2:
                    triplets.add((a, b, c))

    return triplets


def is_triplet(triplet):
    a, b, c = sorted(triplet)
    return a ** 2 + b ** 2 == c ** 2
