from functools import reduce


def largest_product(series, size):
    if size < 0 or size > len(series):
        raise ValueError("invalid span")

    largest = 0

    for i in range(0, len(series) - size + 1):
        product = reduce(lambda acc, c: acc * int(c), series[i : i + size], 1)
        largest = max(largest, product)

    return largest
