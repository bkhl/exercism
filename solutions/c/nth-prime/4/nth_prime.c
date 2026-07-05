#include "nth_prime.h"

#include <math.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>

// https://en.wikipedia.org/wiki/Prime-counting_function#Inequalities
uint32_t get_max_prime(uint32_t n) {
  n = n > 6 ? n : 6;
  return n * (log(n) + log(log(n)));
}

uint32_t nth(uint32_t n) {
  switch (n) {
  case 0: return 0;
  case 1: return 2;
  case 2: return 3;
  }

  size_t max_prime = get_max_prime(n);

  bool *sieve = (bool *)calloc(max_prime, max_prime * sizeof(bool) + 1);

  uint32_t prime = 3;

  for (uint32_t i = 2; i < n; i++) {
    for (size_t non_prime = prime * 2; non_prime <= max_prime; non_prime += prime) {
      sieve[non_prime] = true;
    }
    uint32_t candidate = prime + 2;
    while (sieve[candidate]) {
      candidate += 2;
    }
    prime = candidate;
  }

  free(sieve);

  return prime;
}
