def sum_of_multiples(limit, multiples):
    result = 0

    for i in range(1, limit):
        for m in multiples:
            if i % m == 0:
                result += i
                break

    return result
