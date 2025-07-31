;;; hamming.el --- Hamming (exercism)  -*- lexical-binding: t; -*-

;;; Commentary:

;;; Code:  

(require 'cl-lib)

(defun hamming-distance (dna1 dna2)
  (when (/= (length dna1)
            (length dna2))
    (error "strands not of equal length"))
  (cl-reduce (lambda (acc letters)
               (if (apply '= letters)
                   acc
                 (+ 1 acc)))
             (cl-mapcar 'list dna1 dna2)
             :initial-value 0))

(provide 'hamming)
;;; hamming.el ends here
