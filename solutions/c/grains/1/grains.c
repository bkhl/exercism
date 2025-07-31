#include "grains.h"

uint64_t square(uint8_t index) {
  if (!index)
    return 0;

  uint64_t result = 1;

  for (int i = 1; i < index; ++i)
    result *= 2;

  return result;
}

uint64_t total() {
  return square(65) - 1;
}
