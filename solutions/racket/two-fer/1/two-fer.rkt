#lang racket

(require racket/contract)

(provide
  (contract-out
    [two-fer (->* () (string?) string?)]))

(define (two-fer [name "you"])
  (format "One for ~a, one for me." name))
