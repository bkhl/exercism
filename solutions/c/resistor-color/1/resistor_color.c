#include "resistor_color.h"

const resistor_band_t COLORS[] = {
  BLACK,
  BROWN,
  RED,
  ORANGE,
  YELLOW,
  GREEN,
  BLUE,
  VIOLET,
  GREY,
  WHITE,
};

const resistor_band_t * colors(void) {
  return COLORS;
}

int color_code(resistor_band_t color) {
  return (int) color;
}
