;;; roman-numerals.el --- roman-numerals Exercise (exercism)  -*- lexical-binding: t; -*-

;;; Commentary:

;;; Code:

(require 'cl-lib)

(defconst to-roman--digits
  '((1000 . "M")
    (900 . "CM")
    (500 . "D")
    (400 . "CD")
    (100 . "C")
    (90 . "XC")
    (50 . "L")
    (40 . "XL")
    (10 . "X")
    (9 . "IX")
    (5 . "V")
    (4 . "IV")
    (1 . "I")))

(defun to-roman (value)
  (cl-loop with n = value while (< 0 n) concat
           (cl-loop for (digit-value . roman) in to-roman--digits
                    if (<= digit-value n)
                    do (setq n (- n digit-value))
                    and return roman)))

(provide 'roman-numerals)
;;; roman-numerals.el ends here
