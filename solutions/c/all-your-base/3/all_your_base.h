#pragma once

#include <stddef.h>
#include <stdint.h>

#define DIGITS_ARRAY_SIZE 64

size_t rebase(int8_t digits[], int16_t input_base, int16_t output_base,
              size_t input_length);

int32_t from_base(int8_t digits[], int16_t input_base, size_t input_length);

size_t to_base(int32_t number, int8_t digits[], int16_t output_base);
