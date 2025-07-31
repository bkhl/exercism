#pragma once

#include <stdint.h>

typedef struct {
  double x;
  double y;
} coordinate_t;

uint8_t score(coordinate_t landing_position);
