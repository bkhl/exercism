#pragma once

#include <stdint.h>

typedef struct {
  uint64_t value;
  uint64_t unit;
} resistor_value_t;

typedef uint64_t resistor_band_t;

enum Color {
  BLACK = 0,
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

enum Unit {
  OHMS = 1,
  KILOOHMS = 1000,
  MEGAOHMS = 1000000,
  GIGAOHMS = 1000000000,
};

resistor_value_t color_code(resistor_band_t resistor_band_t[]);
