;;; leap.el --- Leap exercise (exercism)  -*- lexical-binding: t; -*-

;;; Commentary:

;;; Code:

(defun leap-year-p (year)
  (and (= (% year 4) 0)
       (or (not (= (% year 100) 0))
           (= (% year 400) 0))))

(provide 'leap-year-p)
;;; leap.el ends here
