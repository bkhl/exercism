#include <math.h>
#include <stdint.h>

#include "darts.h"

uint8_t score(coordinate_t landing_position) {
  double d = sqrt(pow(landing_position.x, 2) + pow(landing_position.y, 2));

  if (d <= 1) {
    return 10;
  } else if (d <= 5) {
    return 5;
  } else if (d <= 10) {
    return 1;
  } else {
    return 0;
  }
}
