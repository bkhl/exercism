(import (rnrs))

(define (hamming-distance strand-a strand-b)
  (when (not (= (string-length strand-a)
                (string-length strand-b)))
    (error "strands of unequal length"))
  
  (fold-right (λ (a b distance)
                (if (char=? a b)
                    distance
                    (+ distance 1)))
              0
              (string->list strand-a)
              (string->list strand-b)))

