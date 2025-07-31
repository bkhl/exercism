from itertools import chain


def sum_of_multiples(limit, multiples):
    return sum(
        set(chain(*(range(multiple, limit, multiple) for multiple in multiples)))
    )
