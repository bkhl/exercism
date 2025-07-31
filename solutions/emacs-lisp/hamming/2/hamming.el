;;; hamming.el --- Hamming (exercism)  -*- lexical-binding: t; -*-

;;; Commentary:

;;; Code:  

(require 'cl-lib)

(defun hamming-distance (dna1 dna2)
  (unless (length= dna1 (length dna2))
    (error "strands not of equal length"))

  (cl-loop for letter1 across dna1
           for letter2 across dna2
           count (/= letter1 letter2)))

(provide 'hamming)
;;; hamming.el ends here
