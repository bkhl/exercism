;;; two-fer.el --- Two-fer Exercise (exercism)

;;; Commentary:

;;; Code:

(defun two-fer (&optional recipient)
  (let ((recipient (if recipient recipient "you")))
    (format "One for %s, one for me." recipient)))

(provide 'two-fer)
;;; two-fer.el ends here
