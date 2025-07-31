def is_armstrong(number):
    digits = [int(c) for c in str(number)]
    sum_of_powers = sum(d ** len(digits) for d in digits)
    return number == sum_of_powers
