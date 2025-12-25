#include "allergies.h"

bool is_allergic_to(allergen_t allergen, int score) {
  return score & (1 << allergen);
}

allergen_list_t get_allergens(int score) {
  allergen_list_t r;
  r.count = 0;
  for (int i = 0; i < ALLERGEN_COUNT; i++) {
    if (is_allergic_to(i, score)) {
      r.allergens[i] = true;
      r.count++;
    }
  }
  return r;
}
