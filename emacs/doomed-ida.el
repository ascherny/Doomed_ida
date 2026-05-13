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
(defun doomed-ida-arch-file ()
  "Run lilp and insert architecture info as org block."
  (interactive)
  (let* ((file (read-file-name "Binary: "))
         (bin  (doomed-ida--binary))
         (output (with-temp-buffer
                   (call-process bin nil t nil file)
                   (string-trim (buffer-string)))))
    (insert (format "#+begin_example\n%s\n#+end_example\n" output))))

(defun doomed-ida--dispatch (beg end transform)
  "Route TRANSFORM to region BEG..END or to point."
  (if (and beg end)
      (doomed-ida--convert-region beg end transform)
    (doomed-ida--convert-at-point transform)))

(defun doomed-ida--convert-region (beg end transform)
  "Convert region BEG..END using TRANSFORM function."
  (save-excursion
    (goto-char beg)
    (let ((matches '()))
      (while (re-search-forward "0[xX]\\([0-9a-fA-F]+\\)\\|'\\(.\\)'\\|\\([0-9]+\\)" end t)
        (push (list (match-beginning 0)
                    (match-end 0)
                    (or (match-string 1)
                        (match-string 2)
                        (match-string 3))
                    (cond ((match-string 1) :hex)
                          ((match-string 2) :char)
                          (t                :dec)))
              matches))
      (dolist (match matches)
        (funcall transform
                 (nth 0 match)
                 (nth 1 match)
                 (nth 2 match)
                 (nth 3 match))))))

(defun doomed-ida--convert-at-point (transform)
  "Find hex/dec/char at point and apply TRANSFORM."
  (save-excursion
    (skip-chars-backward "0-9a-fA-FxX'")
    (cond
     ((looking-at "0[xX]\\([0-9a-fA-F]+\\)")
      (funcall transform
               (match-beginning 0) (match-end 0)
               (match-string 1) :hex))
     ((looking-at "\\([0-9]+\\)")
      (funcall transform
               (match-beginning 0) (match-end 0)
               (match-string 1) :dec))
     ((looking-at "'\\(.\\)'")
      (funcall transform
               (match-beginning 0) (match-end 0)
               (match-string 1) :char)))))

(defun doomed-ida--transformer-dec (beg end digits kind)
  "Replace BEG..END with hex<->dec conversion."
  (if (eq kind :char)
      (message "Use doomed-ida-num-char for char conversion")
    (let ((new (if (eq kind :hex)
                   (number-to-string (string-to-number digits 16))
                 (format "0x%X" (string-to-number digits)))))
      (doomed-ida--replace beg end new))))

(defun doomed-ida--num-to-char (beg end num)
  "Replace BEG..END with printable char for NUM."
  (let ((ch (when (characterp num) (char-to-string num))))
    (if (and ch (string-match "[[:print:]]" ch))
        (doomed-ida--replace beg end (format "'%s'" ch))
      (message "Non-printable (0x%X = %d)" num num))))

(defun doomed-ida--transformer-char (beg end digits kind)
  "Replace BEG..END with hex<->char conversion.
Handles 0x41 -> 'A' and 'A' -> 0x41.  Ignores dec numbers."
  (cond
   ((eq kind :hex)
    (doomed-ida--num-to-char beg end (string-to-number digits 16)))
   ((eq kind :char)
    (doomed-ida--replace beg end
                         (format "0x%X" (string-to-char digits))))
   ((eq kind :dec)
    (message "Use doomed-ida-hex-dec for dec conversion"))))

(defun doomed-ida--replace (beg end new-text)
  "Replace region BEG..END with NEW-TEXT."
  (delete-region beg end)
  (goto-char beg)
  (insert new-text))

;;;###autoload
(defun doomed-ida-hex-dec (beg end)
  "Convert hex <-> dec at point or in region."
  (interactive (if (use-region-p)
                   (list (region-beginning) (region-end))
                 (list nil nil)))
  (doomed-ida--dispatch beg end #'doomed-ida--transformer-dec))

;;;###autoload
(defun doomed-ida-num-char (beg end)
  "Convert hex <-> char at point or in region."
  (interactive (if (use-region-p)
                   (list (region-beginning) (region-end))
                 (list nil nil)))
  (doomed-ida--dispatch beg end #'doomed-ida--transformer-char))

(provide 'doomed-ida)
;;; doomed-ida.el ends here
