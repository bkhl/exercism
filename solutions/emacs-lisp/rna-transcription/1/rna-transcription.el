;;; rna-transcription.el -- RNA Transcription (exercism)  -*- lexical-binding: t; -*-

;;; Commentary:

;;; Code:

(defconst rna-transcription--map
  '((?A . ?U)
    (?C . ?G)
    (?G . ?C)
    (?T . ?A)))

(defun to-rna (strand)
  (concat (mapcar (lambda (c) (alist-get c rna-transcription--map))
                  strand)))

(provide 'rna-transcription)
;;; rna-transcription.el ends here
