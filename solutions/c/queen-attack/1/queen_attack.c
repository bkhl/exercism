#include <stdlib.h>

#include "queen_attack.h"

bool positions_equal(const position_t *a, const position_t *b) {
  return a->row == b->row && a->column == b->column;
}

bool position_valid(const position_t *p) {
  return p->row < 8 && p->column < 8;
}

attack_status_t can_attack(position_t queen_1, position_t queen_2) {
  if (positions_equal(&queen_1, &queen_2)) {
    return INVALID_POSITION;
  } else if (!position_valid(&queen_1) || !position_valid(&queen_2)) {
    return INVALID_POSITION;
  } else if (queen_1.row == queen_2.row || queen_1.column == queen_2.column) {
    return CAN_ATTACK;
  } else if (abs(queen_1.row - queen_2.row) ==
             abs(queen_1.column - queen_2.column)) {
    return CAN_ATTACK;
  } else {
    return CAN_NOT_ATTACK;
  }
}
