#include "rotational_cipher.h"

#include <ctype.h>
#include <stdlib.h>
#include <string.h>

char *rotate(const char *text, int shift_key) {
  char *result = malloc(strlen(text) + 1);

  char *r = result;

  for (const char *c = text; *c; c++, r++) {
    char b;

    if (isupper(*c)) {
      b = 'A';
    } else if (islower(*c)) {
      b = 'a';
    } else {
      *r = *c;
      continue;
    }

    *r = b + (*c - b + shift_key) % 26;
  }

  *r = '\0';

  return result;
}
