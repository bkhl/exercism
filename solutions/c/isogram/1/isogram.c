#include "isogram.h"

#include "ctype.h"
#include "stdbool.h"

bool is_isogram(const char phrase[]) {
  if (!phrase) {
    return false;
  }

  bool seen_letters[27] = {false};

  for (int i = 0; phrase[i] != 0; i++) {
    int c = toupper(phrase[i]);

    if (isalpha(c)) {
      int letter = c - 64;

      if (seen_letters[letter]) {
        return false;
      }

      seen_letters[letter] = true;
    }
  }

  return true;
}
