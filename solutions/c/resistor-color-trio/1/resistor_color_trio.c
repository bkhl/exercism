#include <math.h>

#include "resistor_color_trio.h"

resistor_value_t color_code(resistor_band_t resistor_bands[]) {
  uint64_t ohms =
      (resistor_bands[0] * 10 + resistor_bands[1]) * pow(10, resistor_bands[2]);

  uint64_t unit;
  if (!ohms) {
    unit = OHMS;
  } else if (ohms % GIGAOHMS == 0) {
    unit = GIGAOHMS;
  } else if (ohms % MEGAOHMS == 0) {
    unit = MEGAOHMS;
  } else if (ohms % KILOOHMS == 0) {
    unit = KILOOHMS;
  } else {
    unit = OHMS;
  }

  uint64_t value = ohms / unit;

  return (resistor_value_t){value, unit};
}
