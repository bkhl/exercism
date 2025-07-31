#lang racket

(provide acronym)

(define (acronym string)
  (list->string
   (map (λ (s) (char-upcase (first (string->list s))))
        (regexp-split #rx"[^A-Za-z\']+" string))))
