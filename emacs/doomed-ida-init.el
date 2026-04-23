;;; doomed-ida-init.el --- setup entrypoint

(add-to-list 'load-path
             (file-name-directory (or load-file-name buffer-file-name)))
(add-to-list 'exec-path (expand-file-name "~/.local/bin"))
;; (setenv "PATH"
;;         (concat (expand-file-name "~/.local/bin") ":"
;;                 (getenv "PATH")))
(require 'doomed-ida)
(provide 'doomed-ida-init)
