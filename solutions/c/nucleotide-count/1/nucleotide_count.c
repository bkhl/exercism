#include "nucleotide_count.h"

#include <stdio.h>
#include <stdlib.h>

char *count(const char *dna_strand) {
  unsigned int A = 0, C = 0, G = 0, T = 0;

  char *r = malloc(20);

  for (const char *d = dna_strand; *d; d++) {
    switch (*d) {
    case 'A':
      A++;
      break;
    case 'C':
      C++;
      break;
    case 'G':
      G++;
      break;
    case 'T':
      T++;
      break;
    default:
      *r = '\0';
      return r;
    }
  }

  sprintf(r, "A:%d C:%d G:%d T:%d", A, C, G, T);

  return r;
}
