#lang racket

(provide nucleotide-counts)

(define (nucleotide-counts dna)
  (let ([counts (make-hash)])
    (for/list ([c (string->list dna)])
      (if (member c '(#\A #\C #\G #\T))
        (hash-set! counts c (+ (hash-ref counts c 0) 1))
        (raise-argument-error)))
    (list (cons #\A (hash-ref counts #\A 0))
          (cons #\C (hash-ref counts #\C 0))
          (cons #\G (hash-ref counts #\G 0))
          (cons #\T (hash-ref counts #\T 0)))))
