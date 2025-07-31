#include "eliuds_eggs.h"

uint32_t egg_count(uint32_t d) {
  uint32_t r = 0;
  for (uint32_t i = d, j = i >> 1; i > 0; i = j, j = i >> 1) {
    if (j << 1 != i) {
      r++;
    }
  }
  return r;
}
