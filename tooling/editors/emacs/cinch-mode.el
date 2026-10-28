;;; cinch-mode.el --- major mode for the Cinch surface syntax -*- lexical-binding: t; -*-

;; Reserved words come from specifications/surface-syntax-and-grammar.md
;; (identical to compiler/src/lexer.rs `keyword`); `handler`, `unsafe`,
;; `when`, `uses`, `immarg`, `for`, `migrate`, `related`, `attributes`,
;; `replaceable`, `specialized` and the target-IR family names are
;; contextual words (usable as identifiers elsewhere).

;;; Code:

(defvar cinch-mode-syntax-table
  (let ((table (make-syntax-table)))
    (modify-syntax-entry ?- ". 12" table)
    (modify-syntax-entry ?\n ">" table)
    (modify-syntax-entry ?{ "(}1nb" table)
    (modify-syntax-entry ?} "){4nb" table)
    (modify-syntax-entry ?_ "w" table)
    (modify-syntax-entry ?' "w" table)
    table)
  "Syntax table for `cinch-mode'.")

(defconst cinch-keywords
  '("module" "import" "export" "as" "data" "record" "codata" "effect"
    "signature" "instance" "implicit" "opaque" "type" "foreign" "ccall"
    "component" "prim" "law" "contract" "rewrite" "target" "upgrade"
    "generate" "infix" "infixl" "infixr" "proof" "spec" "stage" "corec"
    "macro" "forall" "Sigma" "using" "in" "let" "letrec" "where" "of"
    "if" "then" "else" "case" "do" "handle" "with" "match" "replace"
    "require" "justify" "to" "erased" "own" "return" "resume")
  "Reserved words (never identifiers).")

(defconst cinch-conditional-words
  '("safety" "liveness" "recurrence" "stability" "always" "eventually"
    "recur" "stable" "assume")
  "Temporal contract vocabulary (contextual).")

(defconst cinch-contextual-words
  '("handler" "unsafe" "when" "uses" "immarg" "for" "migrate" "related"
    "attributes" "replaceable" "specialized" "llvm" "wasm" "asm" "clr"
    "jvm")
  "Contextual words (ordinary identifiers elsewhere).")

(defconst cinch-font-lock-keywords
  (list
   (cons (regexp-opt cinch-keywords 'symbols) 'font-lock-keyword-face)
   (cons (regexp-opt cinch-conditional-words 'symbols) 'font-lock-keyword-face)
   (cons (regexp-opt cinch-contextual-words 'symbols) 'font-lock-variable-name-face)
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
  "Font-lock rules for `cinch-mode'.")

;;;###autoload
(define-derived-mode cinch-mode prog-mode "Cinch"
  "Major mode for editing Cinch source files."
  :syntax-table cinch-mode-syntax-table
  (setq-local font-lock-defaults '(cinch-font-lock-keywords))
  (setq-local comment-start "-- ")
  (setq-local comment-start-skip "--+\\s-*"))

;;;###autoload
(add-to-list 'auto-mode-alist '("\\.cinch\\'" . cinch-mode))

(provide 'cinch-mode)
;;; cinch-mode.el ends here
