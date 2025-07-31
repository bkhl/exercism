#!usr/bin/env python3


class HighScores(object):
    def __init__(self, scores):
        self.scores = scores

    @property
    def scores(self):
        return self._scores

    @scores.setter
    def scores(self, value):
        self._scores = value
        self._sorted_scores = sorted(value)

    def personal_top_three(self):
        return self._sorted_scores[-1:-4:-1]

    def personal_best(self):
        return self._sorted_scores[-1]

    def latest(self):
        return self.scores[-1]
