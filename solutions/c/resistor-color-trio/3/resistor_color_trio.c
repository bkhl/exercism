#include <math.h>

#include "resistor_color_trio.h"

resistor_value_t color_code(resistor_band_t resistor_bands[]) {
  uint64_t r =
      (resistor_bands[0] * 10 + resistor_bands[1]) * pow(10, resistor_bands[2]);

  uint64_t unit;

  if (r < KILOOHMS) {
    unit = OHMS;
  } else if (r < MEGAOHMS) {
    unit = KILOOHMS;
  } else if (r < GIGAOHMS) {
    unit = MEGAOHMS;
  } else {
    unit = GIGAOHMS;
  }

  return (resistor_value_t){r / unit, unit};
}
