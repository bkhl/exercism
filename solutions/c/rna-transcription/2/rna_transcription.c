#include <stdlib.h>
#include <string.h>

#include "rna_transcription.h"

char to_rna_nucleotide(char c) {
  switch (c) {
  case 'G':
    return 'C';
  case 'C':
    return 'G';
  case 'T':
    return 'A';
  case 'A':
    return 'U';
  default:
    return '\0';
  }
}

char *to_rna(const char *dna) {
  size_t length = strlen(dna) + 1;
  char *rna = malloc(length);
  memset(rna, '\0', length);

  for (size_t i = 0; dna[i]; i++) {
    rna[i] = to_rna_nucleotide(dna[i]);
  }

  return rna;
}
