(import (rnrs))

(define (square n)
  (unless (<= 1 n 64)
    (error "n must be positive integer"))
  (expt 2 (-  n 1)))

(define total
  (- (expt 2 64) 1))
