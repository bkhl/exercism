;;; roman-numerals.el --- roman-numerals Exercise (exercism)  -*- lexical-binding: t; -*-

;;; Commentary:

;;; Code:

(defun to-roman (value)
  "Return a string representing VALUE as a Roman numeral"
  (apply 'string
         (mapcar 'to-roman--digit-to-char
                 (append (make-list (/ value 1000) 1000)
                         (to-roman--partial 100 500 1000 (% value 1000) nil)
                         (to-roman--partial 10 50 100 (% value 100) nil)
                         (to-roman--partial 1 5 10 (% value 10) nil)))))

(defun to-roman--partial (low mid high value result)
  "Return a list of integers representing a partial Roman numeral representation of VALUE,
where LOW/MID/HIGH is a range like 1/5/10 (for I/V/X).
RESULT is an accumaltor that should be initialized to an empty list."
  (let ((digits (cond ((<= high value) `(,high))
                      ((<= (- high low) value) `(,low ,high))
                      ((<= mid value) `(,mid))
                      ((<= (- mid low) value) `(,low ,mid))
                      ((<= low value) `(,low))
                      (t nil))))
    (if (null digits)
        result
      (to-roman--partial low mid high
                         (- value (apply '+ digits))
                         (append result digits)))))

(defun to-roman--digit-to-char (digit)
  "Return a char representing the number DIGIT as a Roman numeral character, or
nil if it does not match one of the characters used in Roman numerals "
  (pcase digit
    (1 ?I)
    (5 ?V)
    (10 ?X)
    (50 ?L)
    (100 ?C)
    (500 ?D )
    (1000 ?M)))

(provide 'roman-numerals)
;;; roman-numerals.el ends here
