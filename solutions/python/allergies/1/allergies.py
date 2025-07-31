class Allergies(object):

    values = {
        "eggs": 1,
        "peanuts": 2,
        "shellfish": 4,
        "strawberries": 8,
        "tomatoes": 16,
        "chocolate": 32,
        "pollen": 64,
        "cats": 128,
    }

    def __init__(self, score):
        self.score = score

    def is_allergic_to(self, item):
        return self.score & self.values[item] == self.values[item]

    @property
    def lst(self):
        return [x for x in self.values.keys() if self.is_allergic_to(x)]
