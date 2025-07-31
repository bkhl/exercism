#lang racket

(require racket/contract)

(provide
  (contract-out
    [square (-> (λ (n) (<= 1 n 64)) positive?)]
    [total (->* () positive?)]))

(define (square n) (expt 2 (- n 1)))

(define (total) (total-by-square 64))

(define (total-by-square n) (- (expt 2 n) 1))
