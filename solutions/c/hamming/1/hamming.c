#include "hamming.h"

int compute(const char *lhs, const char *rhs) {
  int distance = 0;

  for (int i = 0; ; i++) {
    char l = lhs[i];
    char r = rhs[i];

    if (l == 0)
      if (r == 0)
        break;
      else
        return -1;
    else
      if (r == 0)
        return -1;

    if (l != r)
      distance++;
  }

  return distance;
}
