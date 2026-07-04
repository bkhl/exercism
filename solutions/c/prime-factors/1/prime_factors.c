#include "prime_factors.h"

size_t find_factors(uint64_t n,  uint64_t factors[static MAXFACTORS]) {
    uint64_t d = 2;
    size_t i = 0;
    while (n > 1) {
        if (n % d == 0) {
            factors[i] = d;
            n /= d;
            i += 1;
        } else {
            d += 1;
        }
    }
    return i;
}
