from collections import defaultdict
from enum import Enum


def repeat_each(iterable, repeat=1):
    for item in iterable:
        for _ in range(repeat):
            yield item


class Plant(Enum):
    Clover = "C"
    Grass = "G"
    Radishes = "R"
    Violets = "V"


class Garden(object):
    def __init__(self, diagram, students=None):
        self.students = (
            sorted(students)
            if students
            else [
                "Alice",
                "Bob",
                "Charlie",
                "David",
                "Eve",
                "Fred",
                "Ginny",
                "Harriet",
                "Ileana",
                "Joseph",
                "Kincaid",
                "Larry",
            ]
        )

        self.plants_by_student = defaultdict(list)

        for row in diagram.split("\n"):
            for student, character in zip(repeat_each(self.students, repeat=2), row):
                self.plants_by_student[student].append(Plant(character))

    def plants(self, student):
        return [p.name for p in self.plants_by_student[student]]
