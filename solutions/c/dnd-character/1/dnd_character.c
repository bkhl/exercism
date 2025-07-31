#include <math.h>
#include <stdlib.h>

#include "dnd_character.h"

dnd_character_t make_dnd_character(void) {
  int constitution = ability();

  return (dnd_character_t){
      ability(),
      ability(),
      constitution,
      ability(),
      ability(),
      ability(),
      10 + modifier(constitution),
  };
}

int modifier(int score) {
  return floor((score - 10) / 2.);
}

int ability(void) {
  int minimum = 6;
  int result = 0;
  for (int i = 0; i < 4; i++) {
    int throw = rand() % 6 + 1;
	minimum = throw < minimum ? throw : minimum;
    result += throw;
  }
  return result - minimum;
}
