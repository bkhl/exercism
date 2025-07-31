def is_isogram(string):
    l = [c for c in list(string.lower()) if c.isalnum()]
    return len(l) == len(set(l))