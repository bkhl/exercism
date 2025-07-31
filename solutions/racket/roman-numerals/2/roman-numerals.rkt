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
  (define (to-roman-partial high mid low input)
    (let*-values
      ([(high-count leftover) (quotient/remainder input high)]
       [(digits) (build-list high-count (λ (_) high))]

       [(digits leftover) (if (<= (- high low) leftover)
                            (values (append (list high low) digits)
                                    (- leftover (- high low)))
                            (values digits leftover))]

       [(digits leftover) (if (<= mid leftover)
                            (values (cons mid digits)
                                    (- leftover mid))
                            (values digits leftover))]

       [(digits leftover) (if (<= (- mid low) leftover)
                            (values (append (list mid low) digits)
                                    (- leftover (- mid low)))
                            (values digits leftover))]

       [(low-count leftover) (quotient/remainder leftover low)]
       [(digits) (append (build-list low-count (λ (_) low)) digits)])
      (values (reverse digits) leftover)))

  (define (digits-to-string digits)
    (list->string (map (λ (n) (dict-ref numerals n)) digits)))

  (let*-values
    ([(highs leftover) (to-roman-partial 1000 500 100 input)]
     [(mids leftover) (to-roman-partial 100 50 10 leftover)]
     [(lows _) (to-roman-partial 10 5 1 leftover)])
    (digits-to-string (append highs mids lows))))

