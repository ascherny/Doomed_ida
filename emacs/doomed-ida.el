;;; doomed-ida.el --- Binary helpers -*- lexical-binding: t; -*-

;;; Commentary:
;; Utilities for reverse engineering workflow.

;;; Code:
(defgroup doomed-ida nil
  "Binary helpers."
  :group 'tools)

(defcustom doomed-ida-binary "lilp"
  "Executable used for architecture parsing."
  :type 'string
  :group 'doomed-ida)

(defun doomed-ida--binary ()
  (or (executable-find doomed-ida-binary)
      (error "Executable '%s' not found in PATH" doomed-ida-binary)))

;;;###autoload
(defun doomed-ida-hex-dec ()
  "Convert hex <-> dec at point."
  (interactive)
  (save-excursion
    (skip-chars-backward "0-9a-fA-FxX")
    (cond
     ((looking-at "0[xX]\\([0-9a-fA-F]+\\)")
      (let ((num (string-to-number (match-string 1) 16)))
        (delete-region (match-beginning 0) (match-end 0))
        (insert (number-to-string num))))

     ((looking-at "\\([0-9]+\\)")
      (let ((num (string-to-number (match-string 1) 10)))
        (delete-region (match-beginning 0) (match-end 0))
        (insert (format "0x%X" num)))))))

;;;###autoload
(defun doomed-ida-arch-file ()
  "Run lilp and insert architecture info as org block."
  (interactive)
  (let* ((file (read-file-name "Binary: "))
         (bin  (doomed-ida--binary))
         (output (with-temp-buffer
                   (call-process bin nil t nil file)
                   (string-trim (buffer-string)))))
    (insert (format "#+begin_example\n%s\n#+end_example\n" output))))

(provide 'doomed-ida)
;;; doomed-ida.el ends here
