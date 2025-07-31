#include "armstrong_numbers.h"
#include "math.h"

bool is_armstrong_number(int candidate) {
  int digit_count = log10(candidate) + 1;
  int n = candidate;
  int sum = 0;

  while (n > 0) {
    int digit = n % 10;
    n = n / 10;

    int power = 1;
    for (int i = 0; i < digit_count; ++i)
      power *= digit;

    sum += power;
  }

  return sum == candidate;
}
