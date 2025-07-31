#include "grains.h"

uint64_t square(uint8_t index) {
  if (!index || index > MAX_SQUARE)
    return 0;

  return UINT64_C(1) << (index - 1);
}

uint64_t total() {
  return -1;
}
