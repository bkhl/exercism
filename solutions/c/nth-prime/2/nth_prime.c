#include "nth_prime.h"

#include <stddef.h>
#include <stdint.h>
#include <stdbool.h>

uint32_t nth(uint32_t n) {
    switch (n) {
        case 0: return 0;
        case 1: return 2;
    }

    uint32_t p = 2;
    bool s[MAX_PRIME] = {false};

    for (uint32_t i = 1; i < n; i++) {
        for (size_t j = p; j < MAX_PRIME; j += p) {
            s[j] = true;
        }
        while (s[p]) {
            p++;
        }
    }

    return p;
}
