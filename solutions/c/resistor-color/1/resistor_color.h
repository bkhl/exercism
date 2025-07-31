#pragma once

typedef enum {
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
} resistor_band_t;

const resistor_band_t * colors(void);

int color_code(resistor_band_t color);
