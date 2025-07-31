(import (rnrs))

(define (leap-year? year)
  (and (zero? (remainder year 4))
       (or (not (zero? (remainder year 100)))
           (zero? (remainder year 400)))))
