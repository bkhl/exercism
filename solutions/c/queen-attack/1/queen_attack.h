#pragma once

#include <stdint.h>

typedef enum { CAN_NOT_ATTACK, CAN_ATTACK, INVALID_POSITION } attack_status_t;

typedef struct {
  uint8_t row;
  uint8_t column;
} position_t;

bool positions_equal(const position_t *a, const position_t *b);

bool position_valid(const position_t *p);

attack_status_t can_attack(position_t queen_1, position_t queen_2);
