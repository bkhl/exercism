#include "luhn.h"

#include <ctype.h>
#include <stdio.h>
#include <string.h>

bool luhn(const char *num) {
  int sum = 0;
  int count = 0;
  bool even = false;

  for (int i = strlen(num) - 1; i > -1; i--) {
    if (isspace(num[i])) {
      continue;
    }

    if (!isdigit(num[i])) {
      return false;
    }

    int n;
    sscanf(&num[i], "%1d", &n);

    if (even == true) {
      n = n * 2;
      if (n > 9) {
        n = n - 9;
      }
    }

    sum += n;
    count++;
    even = !even;
  }

  if (count < 2) {
    return false;
  }

  if (sum % 10 == 0) {
    return true;
  }

  return false;
}
