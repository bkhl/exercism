#include "sum_of_multiples.h"

unsigned int sum(const unsigned int *factors, const size_t number_of_factors,
                 const unsigned int limit) {
  unsigned int r = 0;

  for (unsigned int i = 1; i < limit; i++) {
    for (size_t j = 0; j < number_of_factors; j++) {
      if (factors[j] == 0) {
        continue;
      }
      if (i % factors[j] == 0) {
        r += i;
        break;
      }
    }
  }

  return r;
}
