def slices(series, length):
    if length < 1:
        raise ValueError("length must be a positive integer")

    if length > len(series):
        raise ValueError("length longer than input series")

    return [series[i : i + length] for i in range(0, len(series) - length + 1)]
