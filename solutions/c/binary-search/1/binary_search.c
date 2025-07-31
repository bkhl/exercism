#include "binary_search.h"

const int *binary_search(int value, const int *arr, size_t length) {
  int a = 0;
  int b = length - 1;

  while (a <= b) {
    int i = (a + b) / 2;
    int v = arr[i];

    if (v < value) {
      a = i + 1;
    } else if (v > value) {
      b = i - 1;
    } else {
      return &arr[i];
    }
  }

  return NULL;
}
