#lang racket

(provide
  (contract-out
    [nucleotide? (-> char? boolean?)]
    [nucleotide-string? (-> string? boolean?)]
    [nucleotide-counts
      (-> nucleotide-string?
          (listof (cons/c nucleotide?  number?)))]))

(define nucleotides '(#\A #\C #\G #\T))

(define (nucleotide? x)
  (member x nucleotides))

(define (nucleotide-string? x)
  (andmap nucleotide? (string->list x)))

#| (define (nucleotide-counts dna) |#
#|   (let ([counts (make-hash)]) |#
#|     (for/list ([nucleotide (string->list dna)]) |#
#|       (hash-set! counts nucleotide (+ (hash-ref counts nucleotide 0) 1))) |#
#|     (map (λ (nucleotide) |#
#|             (cons nucleotide (hash-ref counts nucleotide 0))) |#
#|          nucleotides))) |#

(define (nucleotide-counts dna)
  (foldl
    (λ (counts) counts)
    (map (λ (x) '(x . 0)) nucleotides)
    (string->list dna)))
