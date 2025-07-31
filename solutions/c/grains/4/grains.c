#include "grains.h"

#define MAX_SQUARE 64

uint64_t square(uint8_t index) {
  if (!index || index > MAX_SQUARE)
    return 0;

  return UINT64_C(1) << (index - 1);
}

uint64_t total(void) {
  /* We know that the solution is 2^64 - 1, which happens to be the maximum value of uint64_t. */
  return UINT64_MAX;
}
