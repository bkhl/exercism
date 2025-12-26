#include "sieve.h"

#include <stdbool.h>
#include <stdlib.h>

uint32_t sieve(uint32_t limit, uint32_t *primes, size_t max_primes) {
  /* In the sieve, false will mean prime, so that we can use calloc to
   * initialize it. */
  bool *l = calloc(limit + 1, sizeof(bool));

  uint32_t r = 0;

  for (size_t i = 2; i <= limit && r < max_primes; i++) {
    if (!l[i]) {
      primes[r++] = i;
      for (uint32_t j = i + i; j <= limit; j += i) {
        l[j] = true;
      }
    }
  }

  free(l);

  return r;
}
