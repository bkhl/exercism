#include "gigasecond.h"

void gigasecond(time_t input, char *output, size_t size) {
  time_t t = input + 1000000000;
  strftime(output, size, "%F %T", gmtime(&t));
}
