#include "isogram.h"

#include "stdint.h"

bool is_isogram(const char phrase[]) {
  if (!phrase) {
    return false;
  }

  uint32_t seen_letters = 0;

  for (int i = 0; phrase[i] != 0; i++) {
    int letter = phrase[i];

    if (letter >= 97 && letter <= 122) {
      letter = letter - 64;
    } else if (! (letter >= 65 && letter <= 90)) {
      continue;
    }

    if (seen_letters & 1 << letter) {
      return false;
    }

    seen_letters |= 1 << letter;
  }

  return true;
}
