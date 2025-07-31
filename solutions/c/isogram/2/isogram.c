#include "isogram.h"

#include "ctype.h"

bool is_isogram(const char phrase[]) {
  if (!phrase) {
    return false;
  }

  __uint32_t seen_letters = 0;

  for (int i = 0; phrase[i] != 0; i++) {
    int c = toupper(phrase[i]);

    if (isalpha(c)) {
      int letter = c - 64;

      if (seen_letters & 1 << letter) {
        return false;
      }

      seen_letters |= 1 << letter;
    }
  }

  return true;
}
