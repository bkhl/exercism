#include "beer_song.h"

#include <stdio.h>

static void recite_verse(uint8_t start_bottles, char **song, size_t verse) {
  char *line = song[verse * 3];
  if (start_bottles > 1) {
    sprintf(line, "%d bottles of beer on the wall, %d bottles of beer.",
            start_bottles, start_bottles);
  } else if (start_bottles == 1) {
    sprintf(line, "%d bottle of beer on the wall, %d bottle of beer.",
            start_bottles, start_bottles);
  } else {
    sprintf(line,
            "No more bottles of beer on the wall, no more bottles of beer.");
  }

  line = song[verse * 3 + 1];
  if (start_bottles > 2) {
    sprintf(line,
            "Take one down and pass it around, %d bottles of beer on the wall.",
            start_bottles - 1);
  } else if (start_bottles == 2) {
    sprintf(line,
            "Take one down and pass it around, %d bottle of beer on the wall.",
            start_bottles - 1);
  } else if (start_bottles == 1) {
    sprintf(line, "Take it down and pass it around, no more bottles of beer "
                  "on the wall.");
  } else {
    sprintf(
        line,
        "Go to the store and buy some more, 99 bottles of beer on the wall.");
  }
}

void recite(uint8_t start_bottles, uint8_t take_down, char **song) {
  size_t verse = 0;
  while (take_down-- > 0) {
    recite_verse(start_bottles--, song, verse++);
  }
}
