#include "sieve.h"

#include <stdbool.h>
#include <stdlib.h>

uint32_t sieve(uint32_t limit, uint32_t *primes, size_t max_primes) {
  /* In the sieve, false will mean prime, so that we can use calloc to
   * initialize it. */
  bool *l = calloc(limit + 1, sizeof(bool));

  for (size_t i = 2; i <= limit; i++) {
    if (l[2]) {
      continue;
    }

    for (uint32_t j = i + i; j <= limit; j += i) {
      l[j] = true;
    }
  }

  size_t j = 0;
  for (uint32_t i = 2; i <= limit && j < max_primes; i++) {
    if (!l[i]) {
      primes[j++] = i;
    }
  }

  free(l);

  return j;
}
