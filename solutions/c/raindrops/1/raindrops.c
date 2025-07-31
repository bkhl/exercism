#include <stdio.h>

#include "raindrops.h"

void sound(char **p, int drops, int divisor, char vowel) {
  if (drops % divisor == 0) {
    *p += sprintf(*p, "Pl%cng", vowel);
  }
}

void convert(char result[], int drops) {
  char *p = result;

  sound(&p, drops, 3, 'i');
  sound(&p, drops, 5, 'a');
  sound(&p, drops, 7, 'o');

  if (p == result) {
    sprintf(result, "%d", drops);
  }
}
