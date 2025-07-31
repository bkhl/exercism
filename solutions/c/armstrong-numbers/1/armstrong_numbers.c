#include "armstrong_numbers.h"
#include "math.h"

bool is_armstrong_number(int candidate) {
  int digit_count = _get_digit_count(candidate);
  int n = candidate;
  int sum = 0;

  while (n > 0) {
    int digit = n % 10;
    n = n / 10;
    sum += pow(digit, digit_count);
  }

  return sum == candidate;
}

int _get_digit_count(int n) {
  int c = 0;
  while (n > 0) {
    c++;
    n = n / 10;
  }
  return c;
}
