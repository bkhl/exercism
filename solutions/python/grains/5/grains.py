def square(integer_number):
    if integer_number < 1 or integer_number > 64:
        raise ValueError("no such square")

    return 2 ** (integer_number - 1)

def total():
    return 2 ** 64 - 1
