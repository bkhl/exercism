from functools import reduce
from operator import mul


def largest_product(series, size):
    if size < 0 or size > len(series):
        raise ValueError("invalid span")

    digits = [int(c) for c in series]
    largest = 0

    for i in range(0, len(digits) - size + 1):
        product = reduce(mul, digits[i : i + size], 1)
        largest = max(largest, product)

    return largest
