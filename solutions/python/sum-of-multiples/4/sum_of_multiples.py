def get_terms(multiples, limit):
    state = {x: x for x in multiples}

    while state:
        multiple, term = max(state.items(), key=lambda t: t[1])

        if term >= limit:
            del state[multiple]
            continue

        for other_multiple in [m for m in state if m != multiple]:
            if term % other_multiple == 0:
                state[multiple] += multiple
                break
        else:
            state[multiple] += multiple
            yield term


def sum_of_multiples(limit, multiples):
    return sum(x for x in get_terms(multiples, limit))
