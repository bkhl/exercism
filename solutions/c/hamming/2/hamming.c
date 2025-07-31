#include "hamming.h"

int compute(const char *lhs, const char *rhs) {
  int distance = 0;
  int i;

  for (i = 0; lhs[i] != 0 && rhs[i] != 0; i++) {
    if (lhs[i] != rhs[i])
      distance++;
  }

  if (lhs[i] != 0 || rhs[i] != 0)
    return -1;

  return distance;
}
