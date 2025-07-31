def is_isogram(string):
    l = [c for c in string.lower() if c.isalnum()]
    return len(l) == len(set(l))