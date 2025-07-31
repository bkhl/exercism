from functools import lru_cache

CACHE_MAX_SIZE = 1024


def get_terms(multiples, limit):
    if not multiples:
        return

    state = {x: x for x in multiples}

    @lru_cache(maxsize=max(m for m in multiples if m < CACHE_MAX_SIZE))
    def divisible_by(x, y):
        return x % y == 0

    while state:
        multiple, term = max(state.items(), key=lambda t: t[1])

        if term >= limit:
            del state[multiple]
            continue

        for other_multiple in [m for m in state if m != multiple]:
            if divisible_by(term, other_multiple):
                state[multiple] += multiple
                break
        else:
            state[multiple] += multiple
            yield term


def sum_of_multiples(limit, multiples):
    return sum(x for x in get_terms(multiples, limit))
