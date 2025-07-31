#lang racket

(provide nucleotide-counts)

(define nucleotides '(#\A #\C #\G #\T))

(define (nucleotide-counts dna)
  (let ([counts (make-hash)])
    (for/list ([nucleotide (string->list dna)])
      (if (member nucleotide nucleotides)
        (hash-set! counts nucleotide (+ (hash-ref counts nucleotide 0) 1))
        (raise-argument-error)))
    (map (lambda (nucleotide)
           (cons nucleotide (hash-ref counts nucleotide 0)))
         nucleotides)))
