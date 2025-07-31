#include "space_age.h"

const float earth_year_seconds = 365.25 * 24 * 60 * 60;

const float values[NEPTUNE + 1] = {
    0.2408467, 0.61519726, 1.,        1.8808158,
    11.862615, 29.447498,  84.016846, 164.79132,
};

float age(planet_t planet, int64_t seconds) {
  if (planet > NEPTUNE) {
    return -1;
  }

  return seconds / values[planet] / earth_year_seconds;
}
