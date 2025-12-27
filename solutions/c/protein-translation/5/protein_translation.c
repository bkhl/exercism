#include "protein_translation.h"

#include <string.h>

protein_t protein(const char *const rna) {
  protein_t r = {.count = 0};

  const char *c = rna;

  while (*c && r.count < MAX_AMINO_ACIDS) {
    switch (*c++) {
    case 'A':
      switch (*c++) {
      case 'U':
        switch (*c++) {
        case 'G':
          r.amino_acids[r.count++] = Methionine;
          continue;
        default:
          goto fail;
        }
      default:
        goto fail;
      }
      break;
    case 'U':
      switch (*c++) {
      case 'A':
        switch (*c++) {
        case 'A':
        case 'G':
          goto end;
        case 'C':
        case 'U':
          r.amino_acids[r.count++] = Tyrosine;
          break;
        default:
          goto fail;
        }
        break;
      case 'C':
        switch (*c++) {
        case 'A':
        case 'C':
        case 'G':
        case 'U':
          r.amino_acids[r.count++] = Serine;
          break;
        default:
          goto fail;
        }
        break;
      case 'G':
        switch (*c++) {
        case 'A':
          goto end;
        case 'C':
        case 'U':
          r.amino_acids[r.count++] = Cysteine;
          break;
        case 'G':
          r.amino_acids[r.count++] = Tryptophan;
          break;
        default:
          goto fail;
        }
        break;
      case 'U':
        switch (*c++) {
        case 'U':
        case 'C':
          r.amino_acids[r.count++] = Phenylalanine;
          continue;
        case 'A':
        case 'G':
          r.amino_acids[r.count++] = Leucine;
          continue;
        default:
          goto fail;
        }
        break;
      default:
        goto fail;
      }
      break;
    default:
      goto fail;
    }
  }

end:
  r.valid = true;
  return r;

fail:
  r.valid = false;
  return r;
}
