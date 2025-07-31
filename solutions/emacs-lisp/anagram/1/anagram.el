;;; anagram.el --- Anagram (exercism)  -*- lexical-binding: t; -*-

;;; Commentary:

;;; Code:

(require 'seq)

(defun anagrams-for (subject candidates)
  "Return list of strings from CANDIDATES that are anagrams of SUBJECT."
  (seq-filter (lambda (candidate)
                (and  (string= (anagram--normalize subject)
                               (anagram--normalize candidate))
                      (not (string= (downcase subject)
                                    (downcase candidate)))))
              candidates))

(defun anagram--normalize (word)
    "Return the string WORD as new string with the letters lower-cased and sorted."
    (apply 'string (sort (string-to-list (downcase word)) '<)))

(provide 'anagram)
;;; anagram.el ends here
