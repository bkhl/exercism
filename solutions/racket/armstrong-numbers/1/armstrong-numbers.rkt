#lang racket

(provide armstrong-number?)

(define (armstrong-number? n)
  (let* ([digits (for/list ([c (number->string n)])
                   (string->number (make-string 1 c)))]
         [digit-count (length digits)])
    (= (for/sum ([d digits])
         (expt d digit-count))
       n)))
