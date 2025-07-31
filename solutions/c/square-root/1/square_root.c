#include "square_root.h"

unsigned int square_root(unsigned int radicand) {
    for (unsigned int i = 1;; i++) {
        if (i * i == radicand) {
            return i;
        }
    }
}
