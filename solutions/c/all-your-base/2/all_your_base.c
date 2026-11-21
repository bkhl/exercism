#include "all_your_base.h"
#include <stdint.h>

size_t rebase(int8_t digits[], int16_t input_base, int16_t output_base,
              size_t input_length) {
  if (input_length == 0) {
    return 0;
  }
  if (input_base <= 1 || output_base <= 1) {
    return 0;
  }
  int64_t number = from_base(digits, input_base, input_length);
  size_t output_length = to_base(number, digits, output_base);
  return output_length;
}

int32_t from_base(int8_t digits[], int16_t input_base, size_t input_length) {
  int32_t result = 0;
  int32_t p = 1;
  for (size_t i = 1; i <= input_length; i++, p *= input_base) {
    int8_t digit = digits[input_length - i];
    if (digit < 0 || digit >= input_base) {
      return -1;
    }
    result += p * digit;
  }
  return result;
}

size_t to_base(int32_t number, int8_t digits[], int16_t output_base) {
  if (number == 0) {
    return 1;
  }
  if (number < 0) {
    return 0;
  }
  size_t i = 0;
  int32_t weight = 1;
  while (weight <= number) {
    int32_t next_weight = weight * output_base;
    int32_t remainder = number % next_weight;
    digits[DIGITS_ARRAY_SIZE - 1 - i] = remainder / weight;
    weight = next_weight;
    i++;
  }

  size_t output_length = i;

  size_t offset = DIGITS_ARRAY_SIZE - output_length;
  for (size_t i = 0; i < output_length; i++) {
    digits[i] = digits[i + offset];
  }

  return output_length;
}
