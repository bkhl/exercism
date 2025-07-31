#lang racket

(provide
  (contract-out
    [roman-digit? (-> char? boolean?)]
    [roman-numeral? (-> string? boolean?)]
    [to-roman (-> integer? roman-numeral?)]))

(define numerals
  '((1 . #\I)
    (5 . #\V)
    (10 . #\X)
    (50 . #\L)
    (100 . #\C)
    (500 . #\D)
    (1000 . #\M)))

(define (roman-digit? x)
  (ormap (λ (pair) (eq? (cdr pair) x))
         numerals))

(define (roman-numeral? x)
  (andmap roman-digit? (string->list x)))

(define (to-roman input)
  (define (convert-partial high mid low input)
    (define (process value digits input)
      (let-values
        ([(count leftover) (quotient/remainder input value)])
        (values (append (build-list count (λ (_) value)) digits)
                leftover)))

    (define (process-subtractive higher low digits leftover)
      (if (<= (- higher low) leftover)
        (values (append (list higher low) digits)
                (- leftover (- higher low)))
        (values digits leftover)))

    (define (to-string digits)
      (list->string (map (λ (n)
                            (dict-ref numerals n))
                         (reverse digits))))
    (let*-values
      ([(digits leftover) (process high '() input)]
       [(digits leftover) (process-subtractive high low digits leftover)]
       [(digits leftover) (process mid digits leftover)]
       [(digits leftover) (process-subtractive mid low digits leftover)]
       [(digits leftover) (process low digits leftover)])
      (values (to-string digits) leftover)))
  (let*-values
    ([(highs leftover) (convert-partial 1000 500 100 input)]
     [(mids leftover) (convert-partial 100 50 10 leftover)]
     [(lows _) (convert-partial 10 5 1 leftover)])
    (string-append highs mids lows)))
