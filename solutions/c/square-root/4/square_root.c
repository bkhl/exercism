#include "square_root.h"

unsigned int square_root(unsigned int radicand) {
    unsigned int too_low = 0;
    unsigned int too_high = radicand + 1;

    for (;;) {
        unsigned int guess = (too_low + too_high) / 2;
        unsigned int guess_multiplied = guess * guess;
        if (guess_multiplied < radicand) {
            too_low = guess;
        } else if (radicand < guess_multiplied) {
            too_high = guess;
        } else {
            return guess;
        }
    }
}
