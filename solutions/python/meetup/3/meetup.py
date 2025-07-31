#!/usr/bin/env python

import string
from datetime import date
from itertools import takewhile

WEEKDAYS = {
    "Monday": 0,
    "Tuesday": 1,
    "Wednesday": 2,
    "Thursday": 3,
    "Friday": 4,
    "Saturday": 5,
    "Sunday": 6,
}


class MeetupDayException(Exception):
    pass


def meetup_day(year, month, day_of_the_week, which):
    if which == "last":
        day_range = range(31, 21, -1)
    elif which == "teenth":
        day_range = range(13, 20)
    else:
        offset = int("".join(takewhile(lambda c: c in string.digits, which))) * 7 - 6
        day_range = range(offset, max(offset + 7, 32))

    for day_candidate in day_range:
        try:
            result_candidate = date(year, month, day_candidate)
        except ValueError:
            continue

        if result_candidate.weekday() == WEEKDAYS[day_of_the_week]:
            return result_candidate

    raise MeetupDayException("invalid date")
