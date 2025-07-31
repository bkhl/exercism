#include <string.h>

#include "grade_school.h"

void init_roster(roster_t *roster) {
  roster->count = 0;
}

int compare_students(const student_t *a, const student_t *b) {
  if (a->grade < b->grade) {
    return -1;
  } else if (a->grade > b->grade) {
    return 1;
  } else {
    return strcmp(a->name, b->name);
  }
}

bool add_student(roster_t *roster, char name[], uint8_t grade) {
  if (roster->count >= MAX_STUDENTS || strlen(name) >= MAX_NAME_LENGTH) {
    return false;
  }

  for (size_t i = 0; i < roster->count; i++) {
    if (!strcmp(roster->students[i].name, name)) {
      return false;
    }
  }

  student_t student;
  student.grade = grade;
  strcpy(student.name, name);

  size_t i = roster->count;
  while (i != 0 && compare_students(&student, &roster->students[i - 1]) < 0) {
    roster->students[i] = roster->students[i - 1];
    i--;
  }
  roster->students[i] = student;
  roster->count++;

  return true;
}

roster_t get_grade(roster_t *roster, uint8_t grade) {
  roster_t grade_roster;

  init_roster(&grade_roster);

  for (size_t i = 0; i < roster->count; i++) {
    if (roster->students[i].grade == grade) {
      grade_roster.students[grade_roster.count] = roster->students[i];
      grade_roster.count++;
    } else if (grade < roster->students[i].grade) {
      break;
    }
  }

  return grade_roster;
}
