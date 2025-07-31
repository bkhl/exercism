#include <string.h>

#include "binary.h"

int convert(const char *input) {
  size_t last_index = strlen(input) - 1;

  int result = 0;
  unsigned int multiplier = 1;

  for (int i = last_index; i >= 0; i--) {
    unsigned int digit;

    switch (input[i]) {
    case '0':
      digit = 0;
      break;
    case '1':
      digit = 1;
      break;
    default:
      return INVALID;
    }

    result += digit * multiplier;
    multiplier *= 2;
  }

  return result;
}
