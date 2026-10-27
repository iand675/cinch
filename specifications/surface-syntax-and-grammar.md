# Surface syntax and grammar

Source: <https://chatgpt.com/space/page_18d4c95146a48191a722223f1e8dd18a>  
Exported: 2026-10-22T18:59:36Z

---

Draft 0.2. This chapter and `grammar/surface.ebnf` replace the earlier Rust-like surface sketches. The language remains strict and fully dependently typed. Haskell-like notation does not imply Haskell's evaluation strategy, implicit laziness or unrestricted sharing.

# Core notation

```haskell
module Example where
  twice :: Nat -> Nat
  twice x = x + x

  data Vec (n :: Nat) (a :: Type) where
    Nil  :: Vec 0 a
    Cons :: forall n. a -> Vec n a -> Vec (n + 1) a

  first :: forall a n region.
           Ref region (Vec (n + 1) a) -> Ref region a
  first xs = case xs of
    Cons x rest -> x
```

`f x y` is curried application, `::` introduces a signature, and `case ... of` performs pattern matching. Pattern matching a borrowed value uses checked borrow-pattern elaboration; it does not move fields out of the referent. `Ref` and `Mut` are library-visible reference types with region indices. Functions consume or borrow according to their types; using a Haskell-like surface does not grant implicit duplication of resources.

Kinds, types and values share term syntax. `Type`, `Prop`, `Nat`, equality and type constructors are checked by elaboration rather than by a second restricted type-expression grammar. `Vec (n + 1) a` uses a computed index. `(n :: Nat) -> Vec n a` is a dependent function type; `Sigma (n :: Nat). Vec n a` is a dependent pair. `forall` binds implicit parameters; explicit annotations remain necessary where inference cannot determine them. Unbound signature variables are errors in this draft, rather than silently generalized.

# Strictness and effects

Arguments evaluate left to right, after the function expression. Application associates left, arrows associate right. Partial application creates a closure; the type of each arrow states the effects at that application step. `a -> b ->{IO r} c` is pure when given only the first argument and effectful when given the second.

`a -> b` means a pure total arrow. `a ->{|e} b` has an open effect row. `a ->{Async r, Net r | e} b` has named effects and a row tail. A local definition without a signature may infer effects; an explicit pure signature cannot hide them. `Diverge` remains explicit whenever termination is not established.

There is no `async` declaration modifier or special `await` expression. A suspendable call returns its ordinary result under an `Async` effect. Task handles remain explicit values when work is spawned. Because evaluation is strict, `spawn s (fetch request)` starts evaluating `fetch` too early; write `spawn s (\() -> fetch request)`.

# Layout and lexing

The EBNF consumes explicit `{`, `;` and `}` tokens after a deterministic layout pass. `where`, `of`, `do` and `let` introduce a layout block unless followed by an explicit `{`. The next non-comment token starts the block at its column. At a later logical line start, equal indentation inserts `;`, lower indentation closes nested implicit blocks, and greater indentation continues the current item. The first item receives no inserted separator.

An implicit block must indent farther than its enclosing implicit block. An explicit delimiter shields enclosing layout while it is open; layout blocks begun inside it still use their own indentation. Closing `)`, `]` or an explicit `}` first closes implicit blocks opened inside that delimiter. End of file closes remaining implicit blocks. `in` closes the nearest applicable implicit `let` block and any nested implicit blocks; it must not cross an unmatched explicit delimiter. A `let` statement in `do` has no `in` and closes by indentation. Use explicit braces for empty blocks or a form the layout rule cannot express. There is no parser-error-based insertion rule.

Comments are `--` to end of line and nestable `{- ... -}` blocks. Tabs are rejected in indentation. Line endings normalize to LF. Identifiers are ASCII in this draft; string and character contents are Unicode scalar values. Strings support `\\`, `\"`, `\'`, `\n`, `\r`, `\t`, and `\u{HEX}` with a valid scalar. Comment recognition has priority over symbol identifiers. Reserved punctuation is matched longest first, including `::`, `->`, `<-`, `=>`, `[|`, `|]`, `$(` and multiplicities. Dot is qualification or projection, never decimal punctuation except inside a numeric token. `f(x)` parses as applying `f` to `x`; `f(x,y)` supplies one tuple argument, not two curried arguments.

All quoted alphabetic terminals in the EBNF are reserved. `async`, `await`, `pure`, `spawn`, `join`, `delay`, `force`, `borrow` and `mutBorrow` are ordinary library names, not control syntax. Bare `_` is a pattern wildcard or elaboration hole; a hole cannot survive a deployable build.

# Operators and binding

The parser records an infix chain, then a fixity pass resolves it using the imported, unambiguous operator declarations. Fixity precedences range from 0 to 9; equal-precedence operators must have compatible associativity or require parentheses. There is no default for an undeclared operator. Application and projection bind tighter than infix operators; `$` is reserved right-associative application below all ordinary infix operators. Type annotations and arrows bind more loosely than expression operators. Prefix negation binds to one postfixed atomic term; write `-(f x)` to negate an application result.

The prelude declares arithmetic, comparison, Boolean and composition fixities. Comparisons are nonassociative. Local fixity declarations do not change the interpretation of already imported code. Parenthesized named binders before an arrow are dependent binders; a caller cannot reinterpret `(x :: A) -> B` as a value annotation followed by an arrow.

`%0`, `%1`, `%aff` and `%many` annotate erased, linear, affine and unrestricted arrow usage. These are constraints checked with the resource calculus, not a completed soundness result. Runtime capability parameters use `using (io :: Io r) => ...`. `with (io = selectedIO) in expression` supplies a lexical binding. `f @{io = selectedIO}` supplies an explicit dictionary at a call. No thread-local runtime lookup is implied.

# Declarations and abstractions

`signature` declares an abstraction with abstract types, operations and laws. `module M (S :: Signature) where ...` accepts a module parameter. `instance` implements a signature; `implicit instance` makes it eligible for unique dictionary resolution. Abstract module identity and associated types preserve the OCaml/Backpack-style abstraction boundary. These forms share one elaboration model, rather than creating unrelated module and trait systems.

`proof`, `spec`, `stage` and `corec` decorate signatures and matching equations. `rewrite` and `target rewrite` carry typed metavariable binders, a match, replacement, optional requirements and justification. Such a rule changes optimized program code only after proof checking; it does not extend definitional equality. `foreign import` declares ABI operations with explicit effects. `upgrade` names migration fields checked by the upgrade schema. WIT itself retains its own standard grammar.

Typed quotation is `[| expression |]`, with `$(code)` for a splice. `quoteDecls where ...` quotes declarations. `generate do ...` executes a checked generator. A macro invocation uses a balanced token tree; its expansion is hygienic and must parse and elaborate under this same language. Cross-module expansion does not bypass stage dependency checks.

# Sequencing

All sequencing uses unqualified `do`. Its interpretation is chosen by an in-scope sequencing module, with direct effect sequencing as the defined default. A custom interpretation must be explicit in the enclosing module or signature; the compiler does not infer short-circuiting merely because a result type happens to be `Result`.

`x <- action` binds according to that module, `let x = value` introduces a pure local binding, and the final expression supplies the block's result. Refutable patterns require an explicit case or an interpretation with a declared typed failure operation. A discarded linear result requires a valid consumption operation. The companion runtime and sequencing specification defines the distinction between control effects, result composition and runtime capabilities.

# Validation boundary

The EBNF defines a proposed concrete syntax, with the lexical and layout rules in this chapter. It is not a type checker, parser implementation or mechanized metatheory. Context checks still enforce legal declarations in signatures, complete module implementations, coverage, totality, effect rows, resource usage, module coherence, proof obligations and stage ordering. The grammar deliberately permits some forms that these later checks must reject.

# Complete EBNF

```ebnf
(* Surface grammar 0.2. EBNF after layout insertion and tokenization.
   Quoted strings are terminals. ? ... ? names lexical classes.
   Syntax acceptance is not type, proof, effect or ownership acceptance. *)

program = { module_decl } ;
module_decl = "module", con_path, { module_parameter },
              [ "::", term ], "where", declaration_block ;
module_parameter = "(", con_id, "::", term, ")" ;
declaration_block = "{", [ declaration, { ";", declaration }, [ ";" ] ], "}" ;
declaration = import_decl | signature_decl | module_decl | instance_decl
            | type_decl | data_decl | record_decl | effect_decl
            | value_signature | equation | foreign_decl | rewrite_decl
            | upgrade_decl | generate_decl | fixity_decl ;
import_decl = "import", [ "stage" | "replaceable" | "specialized" ],
              con_path, [ "as", con_id ] ;
signature_decl = "signature", con_id, { binder }, "where", declaration_block ;
instance_decl = [ "implicit" ], "instance", con_id, { binder },
                "::", term, "where", declaration_block ;
type_decl = [ "opaque" ], "type", con_id, { binder },
            ( "::", term | "=", term ) ;
data_decl = "data", con_id, { binder }, [ "::", term ], "where", constructor_block ;
constructor_block = "{", [ constructor_decl, { ";", constructor_decl }, [ ";" ] ], "}" ;
constructor_decl = con_id, "::", term ;
record_decl = ( "record" | "codata" ), con_id, { binder }, "where", field_block ;
field_block = "{", [ field_decl, { ";", field_decl }, [ ";" ] ], "}" ;
field_decl = [ "erased" ], var_id, "::", term ;
effect_decl = "effect", con_id, { binder }, "where", operation_block ;
operation_block = "{", [ operation_decl, { ";", operation_decl }, [ ";" ] ], "}" ;
operation_decl = var_id, "::", term ;
modifier = "proof" | "spec" | "stage" | "corec" | "macro" ;
value_signature = [ modifier ], value_name, "::", term ;
equation = [ modifier ], value_name, { atomic_pattern }, "=", term,
           [ "where", local_block ] ;
value_name = var_id | "(", operator, ")" ;
local_block = "{", local_decl, { ";", local_decl }, [ ";" ], "}" ;
local_decl = value_signature | equation | pattern_binding ;
pattern_binding = pattern, "=", term ;
fixity_decl = ( "infix" | "infixl" | "infixr" ), integer,
              operator, { ",", operator } ;

foreign_decl = "foreign", ( "import" | "export" ),
               ( "ccall" | "component" ), string, var_id, "::", term ;
rewrite_decl = [ "target" ], "rewrite", var_id,
               [ "forall", binder, { binder }, "." ],
               [ using_context ], "where", rewrite_block ;
rewrite_block = "{", "match", term, ";", "replace", term, ";",
                { "require", term, ";" }, "justify", term, [ ";" ], "}" ;
upgrade_decl = "upgrade", con_path, "to", con_path, "where", assignment_block ;
generate_decl = "generate", do_expr ;
assignment_block = "{", assignment, { ";", assignment }, [ ";" ], "}" ;
assignment = var_id, "=", term ;

(* Terms also occur in types. Sorting, relevance and totality are checked later. *)
term = forall_term | sigma_term | contextual_term | lambda | let_term
     | if_term | case_term | do_expr | handle_term | with_term | arrow_term ;
forall_term = "forall", binder, { binder }, ".", term ;
sigma_term = "Sigma", typed_binder, ".", term ;
contextual_term = using_context, "=>", term ;
using_context = "using", "(", capability_binder,
                { ",", capability_binder }, ")" ;
capability_binder = var_id, "::", term ;
binder = var_id | typed_binder | implicit_binder ;
typed_binder = "(", [ "erased" | "own" ], var_id, "::", term, ")" ;
implicit_binder = "{", [ "erased" ], var_id, "::", term, "}" ;
arrow_term = ( typed_binder | implicit_binder ), [ multiplicity ], arrow, term
           | annotation, [ [ multiplicity ], arrow, term ] ;
multiplicity = "%0" | "%1" | "%aff" | "%many" ;
arrow = "->", [ effect_row ] ;
effect_row = "{", [ effect_label, { ",", effect_label } ],
             [ "|", var_id ], "}" ;
effect_label = con_path, { atomic_term } ;
annotation = dollar_expr, [ "::", term ] ;
dollar_expr = infix_expr, [ "$", term ] ;
range_literal = "[", term, [ ",", term ], "..", term, "]" ;
infix_expr = application, { operator, application } ;
application = prefix, { postfix } ;
prefix = { "-" }, postfix ;
postfix = atomic_term, { projection | type_argument | supplied_capabilities } ;
projection = ".", var_id ;
type_argument = "@", atomic_term ;
supplied_capabilities = "@", "{", assignment, { ",", assignment }, "}" ;
atomic_term = var_path | con_path | literal | "_" | "()"
            | "(", term, ")"
            | "(", term, ",", term, { ",", term }, ")"
            | "[", [ term, { ",", term } ], "]"
            | con_path, "{", [ assignment, { ",", assignment } ], "}"
            | "(", operator, ")"
            | quote_expr | splice_expr | handler_expr | macro_call ;
lambda = "\\", atomic_pattern, { atomic_pattern }, "->", term ;
let_term = "let", local_block, "in", term ;
if_term = "if", term, "then", term, "else", term ;
case_term = "case", term, "of", alternative_block ;
alternative_block = "{", alternative, { ";", alternative }, [ ";" ], "}" ;
alternative = pattern, [ "|", term ], "->", term ;
do_expr = "do", "{", { do_statement, ";" }, term, [ ";" ], "}" ;
do_statement = "let", local_block | pattern, "<-", term | term ;
with_term = "with", "(", assignment, { ",", assignment }, ")", "in", term ;
handle_term = "handle", term, "with", term ;
handler_expr = "handler", effect_label, "where", handler_block ;
handler_block = "{", return_clause, { ";", operation_clause }, [ ";" ], "}" ;
return_clause = "return", atomic_pattern, "->", term ;
operation_clause = var_id, { atomic_pattern }, "resume", var_id, "->", term ;
quote_expr = "[|", term, "|]" | "quoteDecls", "where", declaration_block ;
splice_expr = "$(", term, ")" ;
macro_call = var_path, "!", token_tree ;
token_tree = ? balanced token tree in (), [] or {}; nested delimiters match ? ;

pattern = constructor_pattern, [ "::", term ] ;
constructor_pattern = con_path, { atomic_pattern } | atomic_pattern ;
atomic_pattern = var_id | "_" | literal | "()" | con_path
               | "(", pattern, ")"
               | "(", pattern, ",", pattern, { ",", pattern }, ")"
               | "[", [ pattern, { ",", pattern } ], "]"
               | con_path, "{", pattern_field, { ",", pattern_field }, "}"
               | var_id, "@", atomic_pattern ;
pattern_field = var_id, "=", pattern ;

con_path = con_id, { ".", con_id } ;
var_path = { con_id, "." }, var_id ;
var_id = ? ASCII lowercase or underscore followed by letters, digits, underscores or apostrophes; not a keyword or lone underscore ? ;
con_id = ? ASCII uppercase followed by letters, digits, underscores or apostrophes ? ;
operator = ? nonempty symbol identifier, or backtick-delimited value path; excludes reserved punctuation; dollar may continue an operator run (`<$>`) but not start one ? ;
literal = integer | float | string | character ;
integer = ? decimal digits, or 0x hexadecimal digits; internal single underscores allowed ? ;
float = ? digits dot digits with optional e/E signed decimal exponent ? ;
string = ? double-quoted UTF-8 text with defined escapes; no raw newline ? ;
character = ? one Unicode scalar or escape in single quotes ? ;
```

[Documentation overview](../overview.md)

