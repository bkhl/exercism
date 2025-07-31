#include "high_scores.h"

int32_t latest(const int32_t *scores, size_t scores_len) {
  return scores[scores_len - 1];
}

int32_t personal_best(const int32_t *scores, size_t scores_len) {
  int32_t r = scores[0];
  for (size_t i = 1; i < scores_len; i++) {
    r = scores[i] > r ? scores[i] : r;
  }
  return r;
}

size_t personal_top_three(const int32_t *scores, size_t scores_len,
                          int32_t *output) {
  size_t output_len = scores_len < 3 ? scores_len : 3;

  for (size_t i = 0; i < scores_len; i++) {
    for (size_t j = 0; j < output_len; j++) {
      if (output[j] < scores[i]) {
        for (int32_t p = scores[i], q; j < output_len; j++) {
	  q = output[j];
	  output[j] = p;
	  p = q;
	}
      }
    }
  }

  return output_len;
}
