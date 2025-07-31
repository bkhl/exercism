from collections import defaultdict


ONES = 1
TWOS = 2
THREES = 3
FOURS = 4
FIVES = 5
SIXES = 6
YACHT = "YACHT"
FULL_HOUSE = "FULL_HOUSE"
FOUR_OF_A_KIND = "FOUR_OF_A_KIND"
LITTLE_STRAIGHT = "LITTLE_STRAIGHT"
BIG_STRAIGHT = "BIG_STRAIGHT"
CHOICE = "CHOICE"


def score(dice, category):
    result = 0

    if isinstance(category, int):
        for eyes in dice:
            if eyes == category:
                result += category

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
