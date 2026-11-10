;;; cinch-core-mode.el --- major mode for Cinch Core text -*- lexical-binding: t; -*-

;; Cinch Core is the textual Core format specified in design/core-output.md:
;; a header (`core 1`, `module`, `target`, `source`), one
;; `unit <Name> :: <type>` record per definition with indented `@`
;; annotations and a `params` line, a `body` of canonical s-expressions
;; indented two spaces per level, and a closing `end`.
;;
;; The highlighter categories are the ones the format has (section 8.3):
;; unit names, dictionary bindings (`$d.*`, `$static`), ownership classes,
;; annotations, numerals, string and byte literals, the header, and `end`.
;; Every face is a standard font-lock face, so any theme colors the file.

;;; Code:

(defvar cinch-core-mode-syntax-table
  (let ((table (make-syntax-table)))
    ;; `--` starts a line comment and the newline ends it (section 4). Both
    ;; dashes carry the comment flags, so a single `-` (a negative literal,
    ;; or the arrow in a type) is still punctuation.
    (modify-syntax-entry ?- ". 12" table)
    (modify-syntax-entry ?\n "> " table)
    (modify-syntax-entry ?_ "w" table)
    table)
  "Syntax table for `cinch-core-mode'.")

(defconst cinch-core-header-keywords
  '("core" "module" "target" "source")
  "Header words (section 4). `opt-level` is matched separately: `-` is not
a symbol character, so `regexp-opt` cannot anchor it.")

(defconst cinch-core-keywords
  '("unit" "end" "body" "params" "static" "forall" "refine" "with")
  "Record and type words. The form words (`let`, `if`, `case`, ...) are in
`cinch-core-forms' instead, because they only ever appear as an
s-expression head.")

(defconst cinch-core-classes
  '("owned" "borrowed" "lent" "uncounted" "immortal")
  "Ownership classes (section 10).")

(defconst cinch-core-forms
  '("int" "nat" "bytes" "string" "sym" "bool" "unit" "local" "global"
    "ctor" "prim" "foreign" "app" "lam" "dictproj" "proj" "instdict"
    "let" "if" "case" "alt" "pat" "dup" "drop" "release" "boxinit"
    "boxget" "boxclear" "refset" "quote" "splice" "syntaxlit" "demand")
  "Canonical s-expression heads, one per node form in section 8.2.")

(defconst cinch-core-font-lock-keywords
  (list
   ;; Header.
   (cons (regexp-opt cinch-core-header-keywords 'symbols)
         'font-lock-preprocessor-face)
   '("opt-level" (0 font-lock-preprocessor-face))
   '("^module[ \t]+\\([^ \t\n]+\\)" (1 font-lock-type-face))
   '("^target[ \t]+\\([^ \t\n]+\\)" (1 font-lock-constant-face))
   '("sha256:[0-9A-Fa-f]+" (0 font-lock-constant-face))

   ;; The unit name on `unit <Name> :: <type>`; `imenu` indexes the same
   ;; names. Dictionary units are `$d.<Class>.<Type>`, which the binding
   ;; rule below colors.
   '("^unit[ \t]+\\([^ \t:$]+\\)" (1 font-lock-function-name-face))
   '("\\$d\\.[A-Za-z0-9_.]+" (0 font-lock-constant-face))
   '("\\$static\\_>" (0 font-lock-constant-face))

   ;; Record words, ownership classes and annotations.
   (cons (regexp-opt cinch-core-keywords 'symbols) 'font-lock-keyword-face)
   (cons (regexp-opt cinch-core-classes 'symbols) 'font-lock-builtin-face)
   '("@[A-Za-z_]+" (0 font-lock-preprocessor-face))
   ;; A parameter's class prints `@<class>` as well as `@ <class>`, so the
   ;; class word wins inside an annotation; group 1 leaves `@` alone.
   '("@\\(owned\\|borrowed\\|lent\\|uncounted\\|immortal\\)\\_>"
     (1 font-lock-builtin-face))

   ;; S-expression heads.
   (list (concat "(\\_<\\(" (regexp-opt cinch-core-forms) "\\)\\_>")
         '(1 font-lock-keyword-face))

   ;; Literals. A byte buffer is `b"..."`; the `b` marks it and the quoted
   ;; part is a string. Only the `b` is matched here: font-lock drops a
   ;; keyword match that reaches into a string, so a matcher spanning
   ;; `b"..."` would color nothing at all, and Emacs regexps have no
   ;; lookahead to stop at the quote.
   '("(bytes \\(b\\)" (1 font-lock-preprocessor-face))
   '("\"\\(?:[^\"\\\\]\\|\\\\.\\)*\"" (0 font-lock-string-face))
   '("[0-9]+" (0 font-lock-constant-face))

   ;; Operators. Parens are left to the syntax table: Emacs has no
   ;; punctuation face older than 29, and a theme already colors brackets.
   '("\\(?:::\\|->\\|=>\\|[-<>=|:,]\\)" (0 font-lock-builtin-face)))
  "Font-lock rules for `cinch-core-mode'.
Later rules override earlier ones, so the class rule inside an annotation
beats the annotation rule, and the literal rules beat the operator rule.")

(defun cinch-core-imenu-create-index ()
  "Index the `unit` names of the buffer for `imenu'."
  (save-excursion
    (goto-char (point-min))
    (let (index)
      (while (re-search-forward "^unit[ \t]+\\([^ \t:]+\\)" nil t)
        (push (cons (match-string-no-properties 1) (match-beginning 0))
              index))
      (nreverse index))))

;;;###autoload
(define-derived-mode cinch-core-mode prog-mode "Cinch-Core"
  "Major mode for editing Cinch Core files (design/core-output.md)."
  :syntax-table cinch-core-mode-syntax-table
  (setq-local font-lock-defaults '(cinch-core-font-lock-keywords))
  (setq-local comment-start "-- ")
  (setq-local comment-start-skip "--+\\s-*")
  (setq-local comment-end "")
  (setq-local comment-use-syntax t)
  (setq-local imenu-create-index-function #'cinch-core-imenu-create-index)
  (setq-local beginning-of-defun-function
              (lambda () (re-search-backward "^unit[ \t]" nil 'move))))

;;;###autoload
(add-to-list 'auto-mode-alist '("\\.core\\'" . cinch-core-mode))

(provide 'cinch-core-mode)
;;; cinch-core-mode.el ends here
