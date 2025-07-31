class HighScores(object):
    def __init__(self, scores):
        self.scores = scores

    @property
    def sorted_scores(self):
        return sorted(self.scores)

    def personal_top_three(self):
        return self.sorted_scores[-1:-4:-1]

    def personal_best(self):
        return self.sorted_scores[-1]

    def latest(self):
        return self.scores[-1]
