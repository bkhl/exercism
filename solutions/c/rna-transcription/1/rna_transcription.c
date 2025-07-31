#include <stdlib.h>
#include <string.h>

#include "rna_transcription.h"

char *to_rna(const char *dna) {
  size_t length = strlen(dna);

  char *rna = malloc(length);

  for (size_t i = 0; i < length; i++) {
    switch (dna[i]) {
    case 'G':
      rna[i] = 'C';
      break;
    case 'C':
      rna[i] = 'G';
      break;
    case 'T':
      rna[i] = 'A';
      break;
    case 'A':
      rna[i] = 'U';
      break;
    }
  }
  rna[length] = 0;

  return rna;
}
