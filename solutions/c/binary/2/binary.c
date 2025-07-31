#include "binary.h"

int convert(const char *input) {
  int r = 0;

  for (char *c = (char *)input; *c != '\0'; c++) {
    unsigned int d;
    if (*c == '0') {
      d = 0;
    } else if (*c == '1') {
      d = 1;
    } else {
      return INVALID;
    }
    r = r * 2 + d;
  }

  return r;
}
