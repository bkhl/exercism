#lang racket

(provide square total)

(define (square n) (expt 2 (- n 1)))

(define (total-by-square n) (- (expt 2 n) 1))

(define (total) (total-by-square 64))
