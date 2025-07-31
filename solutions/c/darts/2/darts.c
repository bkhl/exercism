#include <math.h>
#include <stdint.h>

#include "darts.h"

uint8_t score(coordinate_t landing_position) {
  double d = hypot(landing_position.x, landing_position.y);

  if (d <= 1.f) {
    return 10;
  } else if (d <= 5.f) {
    return 5;
  } else if (d <= 10.f) {
    return 1;
  } else {
    return 0;
  }
}
