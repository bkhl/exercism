#include "clock.h"

#include <stdio.h>
#include <string.h>

static void read_clock(clock_t clock, int *hour, int *minute) {
  sscanf(clock.text, "%02d:%02d", hour, minute);
}

clock_t clock_create(int hour, int minute) {
  int hour_from_minute = minute / 60;

  minute = minute % 60;
  if (minute < 0) {
    minute += 60;
    hour_from_minute--;
  }

  hour = (hour + hour_from_minute) % 24;
  if (hour < 0) {
    hour += 24;
  }

  clock_t r;
  sprintf(r.text, "%02d:%02d", hour, minute);
  return r;
}

clock_t clock_add(clock_t clock, int minute_add) {
  int hour, minute;
  read_clock(clock, &hour, &minute);
  return clock_create(hour, minute + minute_add);
}

clock_t clock_subtract(clock_t clock, int minute_subtract) {
  int hour, minute;
  read_clock(clock, &hour, &minute);
  return clock_create(hour, minute - minute_subtract);
}

bool clock_is_equal(clock_t a, clock_t b) {
  return strcmp(a.text, b.text) == 0;
}
