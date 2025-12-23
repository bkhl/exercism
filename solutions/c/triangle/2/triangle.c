#include "triangle.h"

bool is_triangle(triangle_t t) {
  return 0 < t.a
    && 0 < t.b
    && 0 < t.c
    && t.a < t.b + t.c
    && t.b < t.a + t.c
    && t.c < t.a + t.b;
}

bool is_equilateral(triangle_t t) {
  return is_triangle(t) && t.a == t.b && t.a == t.c;
}

bool is_isosceles(triangle_t t) {
  return is_triangle(t) && (t.a == t.b || t.a == t.c || t.b == t.c);
}

bool is_scalene(triangle_t t) {
  return is_triangle(t) && t.a != t.b && t.a != t.c && t.b != t.c;
}
