def append(xs, ys):
    return xs + ys


def concat(lists):
    result = []

    for l in lists:
        result += l

    return result


def filter_clone(function, xs):
    return [x for x in xs if function(x)]


def length(xs):
    result = 0

    for _ in xs:
        result += 1

    return result


def map_clone(function, xs):
    return [function(x) for x in xs]


def foldl(function, xs, acc):
    for x in xs:
        acc = function(acc, x)

    return acc


def foldr(function, xs, acc):
    if xs:
        return foldr(function, xs[:-1], function(xs[-1], acc))
    else:
        return acc


def reverse(xs):
    return [x for x in xs[::-1]]
