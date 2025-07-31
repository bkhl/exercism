#include "perfect_numbers.h"

kind classify_number(int n) {
  if (n < 1) {
    return ERROR;
  }

  int s = 0;

  for (int i = 1; i < n; i++) {
    if (n % i == 0) {
      s += i;
    }
  }

  if (s == n) {
    return PERFECT_NUMBER;
  } else if (s < n) {
    return DEFICIENT_NUMBER;
  } else {
    return ABUNDANT_NUMBER;
  }
}
