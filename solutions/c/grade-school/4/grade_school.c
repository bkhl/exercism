#include <string.h>

#include "grade_school.h"

void init_roster(roster_t *roster) {
  roster->count = 0;
}

bool add_student(roster_t *roster, char name[], uint8_t grade) {
  for (size_t i = 0; i < roster->count; i++) {
	if (!strcmp(roster->students[i].name, name)) return false;
  }

  student_t student;
  student.grade = grade;
  strcpy(student.name, name);

  for (size_t i = roster->count;; i--) {
	if (!i) {
	  roster->students[i] = student;
	  break;
	}

	int prev = i - 1;

	student_t registered_student = roster->students[prev];

	if (registered_student.grade < grade
		|| (registered_student.grade == grade
			&& strcmp(name, registered_student.name) > 0))
	{
	  roster->students[i] = student;
	  break;
	} else {
	  roster->students[i] = registered_student;
	}
  }

  roster->count += 1;

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
