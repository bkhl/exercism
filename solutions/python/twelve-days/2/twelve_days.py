#!/usr/bin/env python3

COUNTING_WORDS = [
    "first",
    "second",
    "third",
    "fourth",
    "fifth",
    "sixth",
    "seventh",
    "eighth",
    "ninth",
    "tenth",
    "eleventh",
    "twelfth",
]

GIFTS = [
    "a Partridge in a Pear Tree",
    "two Turtle Doves",
    "three French Hens",
    "four Calling Birds",
    "five Gold Rings",
    "six Geese-a-Laying",
    "seven Swans-a-Swimming",
    "eight Maids-a-Milking",
    "nine Ladies Dancing",
    "ten Lords-a-Leaping",
    "eleven Pipers Piping",
    "twelve Drummers Drumming",
]


def get_gifts(day):
    if day == 1:
        return GIFTS[0]
    else:
        return f"{', '.join(GIFTS[day - 1 : 0 : -1])}, and {GIFTS[0]}"


def recite_verse(day):
    return (
        f"On the {COUNTING_WORDS[day - 1]} day of Christmas my true love "
        f"gave to me: {get_gifts(day)}."
    )


def recite(start_verse, end_verse):
    return [recite_verse(day) for day in range(start_verse, end_verse + 1)]
