from collections import defaultdict
from itertools import chain


class School(object):
    def __init__(self):
        self.students_by_grade = defaultdict(set)

    def add_student(self, name, grade):
        self.students_by_grade[grade].add(name)

    def roster(self):
        return list(
            chain.from_iterable(
                self.grade(grade_number)
                for grade_number in sorted(self.students_by_grade)
            )
        )

    def grade(self, grade_number):
        return sorted(self.students_by_grade.get(grade_number, []))
