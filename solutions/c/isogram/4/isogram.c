#include "isogram.h"

#include "stdint.h"

bool is_isogram(const char phrase[]) {
  if (!phrase) {
    return false;
  }

  uint32_t seen_letters = 0;

  for (int i = 0; phrase[i] != 0; i++) {
    int c = phrase[i];

    if (c >= 'a' && c <= 'z') {
      c = c - 64;
    } else if (! (c >= 'A' && c <= 'Z')) {
      continue;
    }

    if (seen_letters & 1 << c) {
      return false;
    }

    seen_letters |= 1 << c;
  }

  return true;
}
