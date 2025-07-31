#include <ctype.h>
#include <stddef.h>

#include "pangram.h"

#define ALPHABET_SIZE 26

bool is_pangram(const char *sentence) {
  if (!sentence) {
    return false;
  }

  bool used[ALPHABET_SIZE] = {false};

  for (size_t i = 0; sentence[i]; i++) {
    if (isalpha(sentence[i])) {
      used[tolower(sentence[i]) - 'a'] = true;
    }
  }

  for (size_t i = 0; i < ALPHABET_SIZE; i++) {
    if (!used[i]) {
      return false;
    }
  }

  return true;
}
