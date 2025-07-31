#lang racket

(provide leap-year?)

(define (leap-year? year)
  (or (and (equal? 0 (modulo year 4))
           (not (equal? 0 (modulo year 100))))
      (equal? 0 (modulo year 400))))
