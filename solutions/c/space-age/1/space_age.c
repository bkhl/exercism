#include "space_age.h"

float age(planet_t planet, int64_t seconds) {
  float f;

  switch (planet) {
  case MERCURY:
    f = 0.2408467;
    break;
  case VENUS:
    f = 0.61519726;
    break;
  case EARTH:
    f = 1.;
    break;
  case MARS:
    f = 1.8808158;
    break;
  case JUPITER:
    f = 11.862615;
    break;
  case SATURN:
    f = 29.447498;
    break;
  case URANUS:
    f = 84.016846;
    break;
  case NEPTUNE:
    f = 164.79132;
    break;
  default:
    return -1;
  }

  return seconds / f / 60. / 60. / 24. / 365.25;
}
