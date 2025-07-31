from collections import defaultdict
from enum import Enum
from itertools import chain, repeat


DEFAULT_STUDENTS = [
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


class Plant(Enum):
    Clover = "C"
    Grass = "G"
    Radishes = "R"
    Violets = "V"


class Garden(object):
    def __init__(self, diagram, students=None):
        self.students = sorted(students) if students else DEFAULT_STUDENTS

        self.plants_by_student = defaultdict(list)

        for row in diagram.split("\n"):
            for student, character in zip(
                (chain(*(repeat(s, 2) for s in self.students))), row
            ):
                self.plants_by_student[student].append(Plant(character))

    def plants(self, student):
        return [p.name for p in self.plants_by_student[student]]
