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

(define (nucleotide-counts dna)
  (foldl (λ (nucleotide result)
            (map (λ (count)
                    (if (eq? (car count) nucleotide)
                      (cons nucleotide (+ (cdr count) 1))
                      count))
                 result))
         (map (λ (nucleotide) (cons nucleotide 0)) nucleotides)
         (string->list dna)))
