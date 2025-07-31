#include <stddef.h>

#include "robot_simulator.h"

robot_status_t robot_create(robot_direction_t direction, int x, int y) {
  return (robot_status_t){
      .direction = direction,
      .position = (robot_position_t){.x = x, .y = y},
  };
}

void robot_move(robot_status_t *robot, const char *commands) {
  for (size_t i = 0; commands[i]; i++) {
    robot_move_t command = commands[i];

    if (command == ADVANCE) {
      switch (robot->direction) {
      case DIRECTION_NORTH:
        robot->position.y++;
        continue;
      case DIRECTION_EAST:
        robot->position.x++;
        continue;
      case DIRECTION_SOUTH:
        robot->position.y--;
        continue;
      case DIRECTION_WEST:
        robot->position.x--;
        continue;
      default:
        continue;
      }
    } else if (command == TURN_RIGHT) {
      robot->direction++;
    } else if (command == TURN_LEFT) {
      robot->direction--;
    }

    if (robot->direction == DIRECTION_MAX) {
      robot->direction = DIRECTION_DEFAULT;
    } else if (robot->direction > DIRECTION_MAX) {
      robot->direction = DIRECTION_MAX - 1;
    }
  }
}
