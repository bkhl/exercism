#lang racket

(require racket/contract)

(provide
  (contract-out
    [my-reverse (-> string? string?)]))

(define (my-reverse s)
  (list->string (reverse (string->list s))))
