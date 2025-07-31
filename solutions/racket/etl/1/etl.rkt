#lang racket

(provide
  (contract-out
    [letter? (-> string? boolean?)]
    [etl (-> (hash/c positive? (listof letter?))
             (hash/c letter? positive?))]))

(define (letter? x)
  (and (string? x)
       (= (string-length x) 1)))

(define (etl input)
  (make-hash
    (append*
      (hash-map
        input
        (λ (value letters)
           (map (λ (letter)
                   (cons (string-downcase letter) value))
                letters))))))
