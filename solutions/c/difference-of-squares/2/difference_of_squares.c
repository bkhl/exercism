#include "difference_of_squares.h"

unsigned int sum_of_squares(unsigned int number) {
    unsigned int sum = 0;
    while (number != 0) {
        sum += number * number;
        number--;
    }
    return sum;
}

unsigned int square_of_sum(unsigned int number) {
    unsigned int sum = 0;
    while (number != 0) {
        sum += number--;
    }
    return sum * sum;
}

unsigned int difference_of_squares(unsigned int number) {
    unsigned int sum_of_squares_result = sum_of_squares(number);
    unsigned int square_of_sum_result = square_of_sum(number);
    return square_of_sum_result - sum_of_squares_result;
}
