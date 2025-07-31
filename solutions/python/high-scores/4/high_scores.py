#!usr/bin/env python3


def personal_top_three(scores):
    return sorted(scores)[-1:-4:-1]


def personal_best(scores):
    return sorted(scores)[-1]


def latest(scores):
    return scores[-1]
