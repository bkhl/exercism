from functools import lru_cache


def saddle_points(matrix):
    if matrix and [len(row) for row in matrix].count(len(matrix[0])) != len(matrix[0]):
        raise ValueError("irregular matrix")

    result = set()

    @lru_cache()
    def get_column_minima(y):
        column_minimum = min(row[y] for row in matrix)
        return {x for x, row in enumerate(matrix) if row[y] == column_minimum}

    for x, row in enumerate(matrix):
        row_max = max(row)
        for y in (y for y, v in enumerate(row) if v == row_max):
            if x in get_column_minima(y):
                result.add((x, y))

    return result
