#lang racket

(provide nucleotide-counts)

(define nucleotides '(#\A #\C #\G #\T))

(define (nucleotide-counts dna)
  (let ([counts (make-hash)])
    (for/list ([character (string->list dna)])
      (if (member character nucleotides)
        (hash-set! counts character (+ (hash-ref counts character 0) 1))
        (raise-argument-error)))
    (map (lambda (nucleotide)
           (cons nucleotide (hash-ref counts nucleotide 0)))
         nucleotides)))
