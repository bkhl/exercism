#include "rotational_cipher.h"

#include <ctype.h>
#include <stdlib.h>
#include <string.h>

char *rotate(const char *text, int shift_key) {
  size_t l = strlen(text);
  char *r = malloc(l + 1);

  const char *c = text;
  char *c2 = r;

  while (*c != '\0') {
    if (isalpha(*c)) {
      *c2 = *c + shift_key;
      if ((isupper(*c) && !isupper(*c2)) || (islower(*c) && !(islower(*c2)))) {
        *c2 -= 26;
      }
    } else {
      *c2 = *c;
    }
    c++;
    c2++;
  }

  *c2 = '\0';

  return r;
}
