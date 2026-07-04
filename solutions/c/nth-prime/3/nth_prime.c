#include "nth_prime.h"

#include <stddef.h>
#include <stdint.h>
#include <stdbool.h>

uint32_t nth(uint32_t n) {
    switch (n) {
        case 0: return 0;
        case 1: return 2;
        case 2: return 3;
    }

    bool s[MAX_PRIME] = {false};

    uint32_t p = 3;
    for (uint32_t i = 2; i < n; i++) {
        for (size_t j = p; j < MAX_PRIME; j += p) {
              s[j] = true;
        }
        while (s[p]) {
            p += 2;
        }
    }

    return p;
}
