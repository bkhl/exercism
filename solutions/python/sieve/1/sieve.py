def sieve(limit):
    numbers = set(range(2, limit + 1))

    for factor in range(2, limit + 1):
        for product in range(factor * 2, limit + 1, factor):
            numbers.discard(product)

    return sorted(numbers)
