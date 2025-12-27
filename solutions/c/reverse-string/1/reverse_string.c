#include "reverse_string.h"

#include <stdlib.h>

char *reverse(const char *value) {
  const char *c = value;

  for (; *c; c++)
    ;

  char *result = malloc(c - value + 1);
  char *r = result;

  for (c--; c >= value; c--) {
    *r++ = *c;
  }

  *r = '\0';

  return result;
}
