#!/usr/bin/env python3

from collections import defaultdict


ONES = object()
TWOS = object()
THREES = object()
FOURS = object()
FIVES = object()
SIXES = object()

INTEGER_CATEGORIES = [ONES, TWOS, THREES, FOURS, FIVES, SIXES]

YACHT = object()
FULL_HOUSE = object()
FOUR_OF_A_KIND = object()
LITTLE_STRAIGHT = object()
BIG_STRAIGHT = object()
CHOICE = object()


def score(dice, category):
    result = 0

    if category in INTEGER_CATEGORIES:
        value = INTEGER_CATEGORIES.index(category) + 1
        for eyes in dice:
            if eyes is value:
                result += value

    elif category == YACHT:
        if len(set(dice)) == 1:
            result = 50

    elif category == FULL_HOUSE:
        dice_by_eyes = sort_dice_by_eyes(dice)
        if set(dice_by_eyes.values()) == {2, 3}:
            result = sum(dice)

    elif category == FOUR_OF_A_KIND:
        dice_by_eyes = sort_dice_by_eyes(dice)
        for eyes, count in dice_by_eyes.items():
            if count >= 4:
                result = eyes * 4
                break

    elif category == LITTLE_STRAIGHT:
        if set(dice) == {1, 2, 3, 4, 5}:
            result = 30

    elif category == BIG_STRAIGHT:
        if set(dice) == {2, 3, 4, 5, 6}:
            result = 30

    elif category == CHOICE:
        result = sum(dice)

    return result


def sort_dice_by_eyes(dice):
    counts = defaultdict(lambda: 0)

    for eyes in dice:
        counts[eyes] += 1

    return dict(counts)
