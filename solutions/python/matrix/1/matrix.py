class Matrix(object):
    def __init__(self, matrix_string):
        self._matrix = [
            [int(i) for i in l.split(" ")] for l in matrix_string.splitlines()
        ]

    def row(self, index):
        return self._matrix[index - 1]

    def column(self, index):
        index -= 1
        return [row[index] for row in self._matrix]
