WORDS = {
    0: "zero",
    1: "one",
    2: "two",
    3: "three",
    4: "four",
    5: "five",
    6: "six",
    7: "seven",
    8: "eight",
    9: "nine",
    10: "ten",
    11: "eleven",
    12: "twelve",
    13: "thirteen",
    14: "fourteen",
    15: "fifteen",
    16: "sixteen",
    17: "seventeen",
    18: "eighteen",
    19: "nineteen",
    20: "twenty",
    30: "thirty",
    40: "forty",
    50: "fifty",
    60: "sixty",
    70: "seventy",
    80: "eighty",
    90: "ninety",
    100: "hundred",
    1E3: "thousand",
    1E6: "million",
    1E9: "billion",
}


AND = "and"


def say(number):
    if number == 0:
        return WORDS[number]

    if number < 0 or number >= 1E12:
        raise ValueError("number too large")

    phrase = []
    magnitude = 1E9
    remainder = number

    while magnitude > 1:
        quotient, remainder = divmod(remainder, magnitude)

        if quotient > 0:
            phrase.extend(say_hundreds(quotient))
            phrase.append(WORDS[magnitude])

        magnitude //= 1E3

    if remainder > 0:
        hundreds_phrase = say_hundreds(remainder)
        if phrase and AND not in hundreds_phrase:
            phrase.append(AND)
        phrase.extend(hundreds_phrase)

    return " ".join(phrase)


def say_hundreds(number):
    phrase = []

    quotient, remainder = divmod(number, 100)

    if quotient > 0:
        phrase.extend([WORDS[quotient], WORDS[100]])

    if remainder > 0:
        if phrase:
            phrase.append(AND)
        phrase.append(say_tens(remainder))

    return phrase


def say_tens(number):
    phrase = []

    if number > 20:
        quotient, remainder = divmod(number, 10)

        if quotient > 0:
            phrase.append(WORDS[quotient * 10])

        if remainder > 0:
            phrase.append(WORDS[remainder])
    else:
        phrase.append(WORDS[number])

    return "-".join(phrase)
