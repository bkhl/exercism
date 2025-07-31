#include "collatz_conjecture.h"

int steps(int start) {
  if (start < 1) {
	return ERROR_VALUE;
  }

  int n = start;
  int r = 0;

  while (n > 1) {
	if (n % 2 == 0) {
	  n = n / 2;
	} else {
	  n = n * 3 + 1;
	}

	r++;
  }

  return r;
}
