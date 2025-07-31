#lang racket

(provide my-length
         my-reverse
         my-map
         my-filter
         my-fold
         my-append
         my-concatenate)

(define (my-length xs)
  (my-fold
    (λ (_ acc) (+ 1 acc))
    0 xs))

(define (my-reverse xs)
  (my-fold cons '() xs))

(define (my-map f xs)
  (my-reverse
    (my-fold
      (λ (x acc) (cons (f x) acc))
      '() xs)))

(define (my-filter f xs)
  (my-reverse
    (my-fold
      (λ (x acc) (if (f x) (cons x acc) acc))
      '() xs)))

(define (my-fold f acc xs)
  (match xs
    ['() acc]
    [(cons x xs) (my-fold f (f x acc) xs)]))

(define (my-append xs ys)
  (my-fold cons ys (my-reverse xs)))

(define (my-concatenate xs)
  (my-fold my-append '() (my-reverse xs)))
