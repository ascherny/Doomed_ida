;;; doomed_ida.el --- Binary helpers -*- lexical-binding: t; -*-

;;; Commentary:
;; Small utilities for reversing workflow

;;; Code:

(defgroup doomed-ida nil
  "Binary helpers."
  :group 'tools)

(defvar doomed-ida-lil-parser-binary "lil_parser"
  "Path or name of lil_parser binary")

;; HEX<->DEC
;;;###autoload
(defun doomed-ida-hex-dec ()
  "Convert hex <-> dec at point"
  (interactive)
  (save-excursion
    (skip-chars-backward "0-9a-fA-FxX")
    (cond

     ((looking-at "0[xX]\\([0-9a-fA-F]+\\)")
      (let* ((start (match-beginning 0))
             (end (match-end 0))
             (hex (match-string 1))
             (num (string-to-number hex 16)))
        (delete-region start end)
        (insert (number-to-string num))))

     ((looking-at "\\([0-9]+\\)")
      (let* ((start (match-beginning 0))
             (end (match-end 0))
             (num (string-to-number (match-string 0) 10)))
        (delete-region start end)
        (insert (format "0x%X" num))))

     (t
      (message "Not a hex/dec")))))

;; My lil binary parser
;;;###autoload
(defun doomed-ida-arch-file ()
  "Run lil_parser and insert architecture info"
  (interactive)
  (let ((file (read-file-name "Binary: ")))
    (insert
     (string-trim
      (shell-command-to-string
       (format "%s %s"
               doomed-ida-lil-parser-binary
               (shell-quote-argument file)))))))

; keybindings
(map! :leader
      :desc "Hex <-> Dec"
      "i h" #'doomed-ida-hex-dec)

(provide 'doomed-ida)

;;; doomed_ida.el ends here :)
