#include "beer_song.h"

#include <stdio.h>

static const char *FIRST_LINE[] = {
    "%u bottles of beer on the wall, %u bottles of beer.",
    "%u bottle of beer on the wall, %u bottle of beer.",
    "No more bottles of beer on the wall, no more bottles of beer.",
};

static const char *SECOND_LINE[] = {
    "Take one down and pass it around, %u bottles of beer on the wall.",
    "Take one down and pass it around, %u bottle of beer on the wall.",
    "Take it down and pass it around, no more bottles of beer on the wall.",
    "Go to the store and buy some more, 99 bottles of beer on the wall.",
};

static void recite_verse(uint8_t start_bottles, char **song, size_t verse) {
  sprintf(song[verse * 3],
          FIRST_LINE[start_bottles > 1 ? 0 : 2 - start_bottles], start_bottles,
          start_bottles);
  sprintf(song[verse * 3 + 1],
          SECOND_LINE[start_bottles > 2 ? 0 : 3 - start_bottles],
          start_bottles - 1);
}

void recite(uint8_t start_bottles, uint8_t take_down, char **song) {
  size_t verse = 0;
  while (take_down-- > 0) {
    recite_verse(start_bottles--, song, verse++);
  }
}
