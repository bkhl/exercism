#include <stddef.h>

#include "robot_simulator.h"

robot_status_t robot_create(robot_direction_t direction, int x, int y) {
  return (robot_status_t){
      .direction = direction,
      .position = (robot_position_t){.x = x, .y = y},
  };
}

void robot_move(robot_status_t *robot, const char *commands) {
  static int move_x[] = {0, 1, 0, -1};
  static int move_y[] = {1, 0, -1, 0};

  for (size_t i = 0; commands[i]; i++) {
    switch (commands[i]) {
    case TURN_RIGHT:
      robot->direction = (robot->direction + 1) % 4;
      break;
    case TURN_LEFT:
      robot->direction = (robot->direction - 1) % 4;
      break;
    case ADVANCE:
      robot->position = (robot_position_t){
          .x = robot->position.x + move_x[robot->direction],
          .y = robot->position.y + move_y[robot->direction],
      };
      break;
    default:
      break;
    }
  }
}
