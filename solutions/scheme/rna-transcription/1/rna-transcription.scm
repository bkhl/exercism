(import (rnrs))

(define (dna->rna dna)
  (string-map (λ (c) (case c
                       ((#\C) #\G)
                       ((#\G) #\C) 
                       ((#\T) #\A)
                       ((#\A) #\U)))
              dna))
