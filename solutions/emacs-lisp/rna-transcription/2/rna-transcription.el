;;; rna-transcription.el -- RNA Transcription (exercism)  -*- lexical-binding: t; -*-

;;; Commentary:

;;; Code:

(defconst rna-transcription--map
  '((?A . ?U)
    (?C . ?G)
    (?G . ?C)
    (?T . ?A)))

(defun to-rna-nucleotide (dna-nucleotide)
  (let ((rna-nucleotide (alist-get dna-nucleotide
                                   rna-transcription--map)))
    (unless rna-nucleotide
      (error "invalid DNA nucleotide %c" dna-nucleotide))
    rna-nucleotide))

(defun to-rna (strand)
  (concat (mapcar 'to-rna-nucleotide strand)))

(provide 'rna-transcription)
;;; rna-transcription.el ends here
