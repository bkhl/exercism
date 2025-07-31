#lang racket

(provide
  (contract-out
    [convert (-> integer? string?)]))

(define sounds '((3 . "Pling")
                 (5 . "Plang")
                 (7 . "Plong")))

(define (convert input)
  (let ([output (filter-map (λ (sound)
                               (if (= (remainder input (car sound)) 0)
                                 (cdr sound)
                                 #f))
                            sounds)])
    (if (null? output)
      (format "~v" input)
      (string-append* output))))
