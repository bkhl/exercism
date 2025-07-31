#!/usr/bin/env python3

"""
Calculate the date of events that occur on the Nth occurance of a certain
weekday in a month.
"""

from datetime import date

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
    """
    Exception raised if it's not possible to find a date matching the criteria.
    """


def meetup_day(year, month, day_of_the_week, which):
    """
    Return a date object for the `day_of_the_week` within the given
    `year`/`month` described by `which`.

    `which` is either the special strings "last" or "teenth", or the first
    character is a digit indicating which occurance of the weekday in the month.
    """

    if which == "last":
        day_range = range(31, 21, -1)
    elif which == "teenth":
        day_range = range(13, 20)
    else:
        day_range = range(int(which[0]) * 7 - 6, 32)

    for day_candidate in day_range:
        try:
            result_candidate = date(year, month, day_candidate)
        except ValueError:
            continue

        if result_candidate.weekday() == WEEKDAYS[day_of_the_week]:
            return result_candidate

    raise MeetupDayException("invalid date")
