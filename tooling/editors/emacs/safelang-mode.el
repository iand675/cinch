;;; safelang-mode.el --- major mode for the SafeLang surface syntax -*- lexical-binding: t; -*-

;; Reserved words come from specifications/surface-syntax-and-grammar.md
;; (identical to compiler/src/lexer.rs `keyword`); `handler`, `unsafe`,
;; `when`, `uses`, `immarg`, `for`, `migrate`, `related`, `attributes`,
;; `replaceable`, `specialized` and the target-IR family names are
;; contextual words (usable as identifiers elsewhere).

;;; Code:

(defvar safelang-mode-syntax-table
  (let ((table (make-syntax-table)))
    (modify-syntax-entry ?- ". 12" table)
    (modify-syntax-entry ?\n ">" table)
    (modify-syntax-entry ?{ "(}1nb" table)
    (modify-syntax-entry ?} "){4nb" table)
    (modify-syntax-entry ?_ "w" table)
    (modify-syntax-entry ?' "w" table)
    table)
  "Syntax table for `safelang-mode'.")

(defconst safelang-keywords
  '("module" "import" "export" "as" "data" "record" "codata" "effect"
    "signature" "instance" "implicit" "opaque" "type" "foreign" "ccall"
    "component" "prim" "law" "contract" "rewrite" "target" "upgrade"
    "generate" "infix" "infixl" "infixr" "proof" "spec" "stage" "corec"
    "macro" "forall" "Sigma" "using" "in" "let" "letrec" "where" "of"
    "if" "then" "else" "case" "do" "handle" "with" "match" "replace"
    "require" "justify" "to" "erased" "own" "return" "resume")
  "Reserved words (never identifiers).")

(defconst safelang-conditional-words
  '("safety" "liveness" "recurrence" "stability" "always" "eventually"
    "recur" "stable" "assume")
  "Temporal contract vocabulary (contextual).")

(defconst safelang-contextual-words
  '("handler" "unsafe" "when" "uses" "immarg" "for" "migrate" "related"
    "attributes" "replaceable" "specialized" "llvm" "wasm" "asm" "clr"
    "jvm")
  "Contextual words (ordinary identifiers elsewhere).")

(defconst safelang-font-lock-keywords
  (list
   (cons (regexp-opt safelang-keywords 'symbols) 'font-lock-keyword-face)
   (cons (regexp-opt safelang-conditional-words 'symbols) 'font-lock-keyword-face)
   (cons (regexp-opt safelang-contextual-words 'symbols) 'font-lock-variable-name-face)
   ;; dash identifiers glue tight (`int-digits-into` is one name)
   '("\\<[a-z_][A-Za-z0-9_']*\\(?:-[A-Za-z_'][A-Za-z0-9_']*\\)*\\>" . font-lock-variable-name-face)
   '("\\<[A-Z][A-Za-z0-9_']*\\>" . font-lock-type-face)
   ;; tick syntax (D123): 'a' is a Char, 'foo a Sym, ''T a type Name
   '("'\\([^'\\]\\|\\\\[\\\\\"'nrt]\\|\\\\u{[0-9A-Fa-f]+}\\)'" . font-lock-string-face)
   '("''[A-Za-z_][A-Za-z0-9_']*" . font-lock-type-face)
   '("'[A-Za-z_][A-Za-z0-9_']*\\(?:-[A-Za-z_'][A-Za-z0-9_']*\\)*" . font-lock-constant-face)
   '("\\_<-?[0-9][0-9_]*\\(?:\\.[0-9][0-9_]*\\)?\\(?:[eE][+-]?[0-9]+\\)?\\_>" . font-lock-constant-face)
   '("#\\[[A-Za-z_][A-Za-z0-9_.-]*" . font-lock-preprocessor-face)
   '("\\_<\\(?:[!#$%&*+/<=>?@^|~:-]\\|\\$\\)+\\_>" . font-lock-variable-name-face))
  "Font-lock rules for `safelang-mode'.")

;;;###autoload
(define-derived-mode safelang-mode prog-mode "SafeLang"
  "Major mode for editing SafeLang source files."
  :syntax-table safelang-mode-syntax-table
  (setq-local font-lock-defaults '(safelang-font-lock-keywords))
  (setq-local comment-start "-- ")
  (setq-local comment-start-skip "--+\\s-*"))

;;;###autoload
(add-to-list 'auto-mode-alist '("\\.lang\\'" . safelang-mode))

(provide 'safelang-mode)
;;; safelang-mode.el ends here
