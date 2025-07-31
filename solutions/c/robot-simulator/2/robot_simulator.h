#pragma once

typedef enum {
  DIRECTION_NORTH = 0,
  DIRECTION_DEFAULT = DIRECTION_NORTH,
  DIRECTION_EAST,
  DIRECTION_SOUTH,
  DIRECTION_WEST,
  DIRECTION_MAX
} robot_direction_t;

typedef struct {
  int x;
  int y;
} robot_position_t;

typedef struct {
  robot_direction_t direction;
  robot_position_t position;
} robot_status_t;

typedef enum {
  TURN_RIGHT = 'R',
  TURN_LEFT = 'L',
  ADVANCE = 'A',
} robot_move_t;

robot_status_t robot_create(robot_direction_t direction, int x, int y);
void robot_move(robot_status_t *robot, const char *commands);
