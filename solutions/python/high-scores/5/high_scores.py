#!usr/bin/env python3


def personal_top_three(scores):
    return sorted(scores, reverse=True)[:3]


def personal_best(scores):
    return sorted(scores)[-1]


def latest(scores):
    return scores[-1]
