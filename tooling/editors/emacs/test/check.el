;;; check.el --- batch checks for cinch-core-mode -*- lexical-binding: t; -*-

;;   emacs -Q --batch -l tooling/editors/emacs/cinch-core-mode.el \
;;         -l tooling/editors/emacs/test/check.el
;;
;; Asserts the mode is a `prog-mode' derivative, that the `auto-mode-alist'
;; entry exists, that `font-lock-keywords' is non-empty, that `imenu' indexes
;; every unit, that `--` is a comment, and that each highlighter category
;; lands on the expected standard face in test/fixture.core. Prints one line
;; per check and exits nonzero when any check failed.

(let* ((test (file-name-directory (or load-file-name buffer-file-name)))
       (mode (expand-file-name "../cinch-core-mode.el" test))
       (fixture (expand-file-name "fixture.core" test))
       (failures 0))

  (unless (featurep 'cinch-core-mode)
    (load mode nil t))

  (defun cc-report (name actual expected)
    (let ((ok (equal actual expected)))
      (princ (format "%-24s %s %S\n" name (if ok "ok  " "FAIL") actual))
      (unless ok (setq failures (1+ failures)))))

  (defun cc-face-at (regexp group &optional offset)
    "Face of the text at REGEXP's GROUP, shifted by OFFSET characters."
    (goto-char (point-min))
    (when (re-search-forward regexp nil t)
      (get-text-property (+ (match-beginning group) (or offset 0)) 'face)))

  ;; The `.core` association is the whole of the file-type support.
  (cc-report "auto-mode-alist"
             (cdr (assoc "\\.core\\'" auto-mode-alist))
             'cinch-core-mode)

  (with-temp-buffer
    (insert-file-contents fixture)
    (cinch-core-mode)
    (font-lock-ensure)

    (cc-report "major mode" major-mode 'cinch-core-mode)
    (cc-report "derived from prog-mode" (and (derived-mode-p 'prog-mode) t) t)
    (cc-report "font-lock-keywords" (and font-lock-keywords t) t)
    (cc-report "comment-start" comment-start "-- ")

    ;; imenu over the unit names, in file order.
    (cc-report "imenu units"
               (mapcar #'car (cinch-core-imenu-create-index))
               '("Main.map" "Main.literals" "Main.scrutinee" "$d.Ord.Nat"
                 "Main.owned" "Main.misc" "Main.quoted"))

    ;; Header.
    (cc-report "header keyword" (cc-face-at "^core" 0)
               'font-lock-preprocessor-face)
    (cc-report "module name" (cc-face-at "^module +\\([^ \t\n]+\\)" 1)
               'font-lock-type-face)
    (cc-report "target triple" (cc-face-at "^target +\\([^ \t\n]+\\)" 1)
               'font-lock-constant-face)
    (cc-report "source digest" (cc-face-at "sha256:" 0)
               'font-lock-constant-face)

    ;; Units, classes, annotations, dictionary bindings.
    (cc-report "unit keyword" (cc-face-at "^unit" 0) 'font-lock-keyword-face)
    (cc-report "unit name" (cc-face-at "^unit +\\([^ \t:]+\\)" 1)
               'font-lock-function-name-face)
    (cc-report "annotation" (cc-face-at "@span" 0) 'font-lock-preprocessor-face)
    (cc-report "class value" (cc-face-at "@class \\(owned\\)" 1)
               'font-lock-builtin-face)
    (cc-report "param class" (cc-face-at "params:.*@\\(borrowed\\)" 1)
               'font-lock-builtin-face)
    (cc-report "dict binding" (cc-face-at "\\$d\\.Ord\\.Nat" 0)
               'font-lock-constant-face)
    (cc-report "end" (cc-face-at "^end" 0) 'font-lock-keyword-face)

    ;; Forms and literals.
    (cc-report "form head" (cc-face-at "(\\(case\\) (local" 1)
               'font-lock-keyword-face)
    (cc-report "int literal" (cc-face-at "(int -?\\(42\\)" 1)
               'font-lock-constant-face)
    (cc-report "string literal" (cc-face-at "(string \\(\"\\)" 1)
               'font-lock-string-face)
    (cc-report "bytes marker" (cc-face-at "(bytes \\(b\\)\"" 1)
               'font-lock-preprocessor-face)
    (cc-report "type arrow" (cc-face-at "forall a b. (a \\(->\\) b)" 1)
               'font-lock-builtin-face)

    ;; `--` starts a comment; a lone `-` does not.
    (goto-char (point-min))
    (re-search-forward "^-- Core")
    (cc-report "line comment" (and (nth 4 (syntax-ppss)) t) t)
    (goto-char (point-min))
    (re-search-forward "(int -42)")
    (cc-report "minus is not a comment"
               (and (nth 4 (syntax-ppss (+ (match-beginning 0) 5))) t)
               nil))

  (kill-emacs (if (> failures 0) 1 0)))
