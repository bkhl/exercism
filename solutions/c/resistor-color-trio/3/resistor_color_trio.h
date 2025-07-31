#pragma once

#include <stdint.h>

typedef struct {
  uint64_t value;
  uint64_t unit;
} resistor_value_t;

typedef enum {
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
} resistor_band_t;

typedef enum {
  OHMS = 1,
  KILOOHMS = 1000,
  MEGAOHMS = 1000000,
  GIGAOHMS = 1000000000,
} resistory_value_unit_t;

resistor_value_t color_code(resistor_band_t resistor_band_t[]);
