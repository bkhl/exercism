#lang racket

(provide acronym)

(define (acronym string)
  (list->string
   (map (λ (s) (char-upcase (first (string->list s))))
        (string-split string #rx"[^A-Za-z\']+"))))
