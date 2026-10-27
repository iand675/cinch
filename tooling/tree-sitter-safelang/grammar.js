// SafeLang tree-sitter grammar.
//
// Normative authorities (see ../../specifications/ and ../../compiler/CONFORMANCE.md):
//   - specifications/surface-syntax-and-grammar.md (draft 0.2 EBNF)
//   - specifications/surface-grammar-local-amendments.md (sections 1-13)
//   - compiler/CONFORMANCE.md decisions D102-D129 (dashes/minus/letrec),
//     D122-D124 (tick syntax), D143-D155 (exports, dollar-in-operators),
//     D198-D214/D221-D227 (contracts, prim bodies, unsafe, when arch).
//
// Design notes (full discussion in tooling/README.md):
//
// * All terminals are produced by the external scanner (src/scanner.c), a
//   port of compiler/src/lexer.rs and compiler/src/layout.rs. That is the
//   only way to reproduce the context-sensitive lexical rules exactly:
//   dash identifiers versus the minus whitespace contract (D102/D128/D129),
//   negative literals, the tick Char-vs-Symbol rule, `$` continuing but
//   never starting an operator run, comment priority over symbol runs, and
//   the layout pass (virtual `{`, `;`, `}` around `where`/`of`/`do`/`let`
//   blocks, `in` closing `let` blocks, `with (...) in` supply clauses).
//
// * Infix chains use the documented prelude/base-library fixity table as
//   precedence layers (prelude `infixl 9 * ...` through `infixr 1 ||`, plus
//   `<>`, `<*>`, `<$>`, `<|>`, `>>=` from the base library). Operators
//   outside that table parse in one flat layer directly above `$` and are
//   resolved by the compiler's fixity pass from `fixity` declarations.
//   Equal-precedence operators with clashing associativity (level 6 `&`
//   vs `<>`, level 3 `<|>` vs the comparisons, level 1 `>>=` vs `||`)
//   may only appear in separate parenthesized chains in valid programs
//   (surface grammar local amendments section 6), so the split between the
//   two sub-levels of each is observable only on programs the compiler's
//   fixity pass rejects.

/// Build one layout block rule: either an implicit block (scanner-inserted
/// `_layout_open` ... `_layout_close`, with `_layout_semi` separators) or an
/// explicit brace block (`{ ... }` with real `;` separators).
function block($, item) {
  return choice(
    seq($._layout_open, repeat(choice(item, $._layout_semi)), $._layout_close),
    seq($.lbrace, repeat(choice(item, $.semicolon)), $.rbrace),
  );
}

/// One left-associative infix layer: `operand (op operand)*`, or just
/// the operand.
function infixLeft($, precedence, ops, operand) {
  return choice(
    operand,
    prec.left(
      precedence,
      seq(
        operand,
        choice(...ops.map((op) => $[op])),
        operand,
        repeat(seq(choice(...ops.map((op) => $[op])), operand)),
      ),
    ),
  );
}

/// One right-associative infix layer.
function infixRight($, precedence, ops, operand) {
  return choice(
    operand,
    prec.right(
      precedence,
      seq(
        operand,
        choice(...ops.map((op) => $[op])),
        operand,
        repeat(seq(choice(...ops.map((op) => $[op])), operand)),
      ),
    ),
  );
}

/// One non-associative infix layer: a single operator, no chaining.
function infixNonassoc($, precedence, ops, operand) {
  return choice(
    operand,
    prec(precedence, seq(operand, choice(...ops.map((op) => $[op])), operand)),
  );
}

// The documented fixity table (prelude/Prelude.lang and the base library).
// Higher numbers bind tighter.
const PREC = {
  mul: 90, // infixl 9 *, /, %
  add: 80, // infixl 8 +, -
  shift: 70, // infixl 7 <<, >>
  band: 61, // infixl 6 &        (sub-level of 6; `<>` is 60)
  concat: 60, // infixr 6 <>
  bxor: 50, // infixl 5 ^
  bor: 40, // infixl 4 |, <*>, <$>
  alt: 31, // infixl 3 <|>      (sub-level of 3; comparisons are 30)
  cmp: 30, // infix  3 ==, /=, <, <=, >, >= (nonassociative)
  and: 20, // infixr 2 &&
  bind: 11, // infixl 1 >>=     (sub-level of 1; `||` is 10)
  or: 10, // infixr 1 ||
  generic: 5, // undeclared operators: one flat layer
  dollar: 1, // `$`: reserved right-associative application
  prefix: 95, // prefix `-`: one postfixed atomic term
};

module.exports = grammar({
  name: 'safelang',

  externals: ($) => [
    // Comments are trivia (hidden, in extras).
    $._comment,

    // Layout (virtual tokens inserted by the scanner; hidden).
    $._layout_open,
    $._layout_semi,
    $._layout_close,

    // Names.
    $.identifier,
    $.constructor,
    $.wildcard,

    // Reserved words (compiler/src/lexer.rs `keyword`).
    $.kw_module,
    $.kw_where,
    $.kw_data,
    $.kw_record,
    $.kw_codata,
    $.kw_effect,
    $.kw_signature,
    $.kw_instance,
    $.kw_implicit,
    $.kw_import,
    $.kw_as,
    $.kw_type,
    $.kw_opaque,
    $.kw_forall,
    $.kw_sigma,
    $.kw_using,
    $.kw_in,
    $.kw_let,
    $.kw_letrec,
    $.kw_if,
    $.kw_then,
    $.kw_else,
    $.kw_case,
    $.kw_of,
    $.kw_do,
    $.kw_handle,
    $.kw_with,
    $.kw_rewrite,
    $.kw_prim,
    $.kw_law,
    $.kw_target,
    $.kw_justify,
    $.kw_match,
    $.kw_replace,
    $.kw_require,
    $.kw_upgrade,
    $.kw_contract,
    $.kw_to,
    $.kw_generate,
    $.kw_quotedecls,
    $.kw_foreign,
    $.kw_export,
    $.kw_ccall,
    $.kw_component,
    $.kw_proof,
    $.kw_spec,
    $.kw_stage,
    $.kw_corec,
    $.kw_macro,
    $.kw_infix,
    $.kw_infixl,
    $.kw_infixr,
    $.kw_erased,
    $.kw_own,
    $.kw_return,
    $.kw_resume,

    // Contextual words: ordinary identifiers elsewhere (D221-D227:
    // `unsafe` is recognized contextually and stays usable as a name).
    $.kw_handler,
    $.kw_unsafe,
    $.kw_when,
    $.kw_uses,
    $.kw_immarg,
    $.kw_for,
    $.kw_safety,
    $.kw_liveness,
    $.kw_recurrence,
    $.kw_stability,
    $.kw_always,
    $.kw_eventually,
    $.kw_recur,
    $.kw_stable,
    $.kw_assume,
    $.kw_migrate,
    $.kw_related,
    $.kw_attributes,
    $.kw_replaceable,
    $.kw_specialized,
    $.kw_llvm,
    $.kw_wasm,
    $.kw_asm,
    $.kw_clr,
    $.kw_jvm,

    // Reserved punctuation.
    $.colon_colon,
    $.arrow,
    $.larrow,
    $.darrow,
    $.lquote, // `[|`
    $.rquote, // `|]`
    $.splice_open, // `$(` (one token: adjacency matters)
    $.mult_zero,
    $.mult_one,
    $.mult_aff,
    $.mult_many,
    $.bang,
    $.at,
    $.dot,
    $.equals,
    $.pipe,
    $.dollar,
    $.backslash,
    $.tilde,

    // Delimiters and separators.
    $.lparen,
    $.rparen,
    $.lbracket,
    $.rbracket,
    $.lbrace,
    $.rbrace,
    $.comma,
    $.semicolon,

    // Attributes: `#[` name [args] `]` (args captured raw, as in the lexer).
    $.attr_open,
    $.attr_name,
    $.attr_args,

    // Literals.
    $.int_literal,
    $.float_literal,
    $.string_literal,
    $.char_literal,
    $.symbol_literal, // `'foo` (Sym; D123)
    $.type_name_literal, // `''T` (type Name; D123)

    // Operators. The fixity-table spellings get their own symbols so the
    // documented precedence can be encoded; everything else is `operator`.
    $.operator,
    $.op_star,
    $.op_slash,
    $.op_percent,
    $.op_plus,
    $.op_minus,
    $.op_ltlt,
    $.op_gtgt,
    $.op_amp,
    $.op_caret,
    $.op_ltgt, // `<>`
    $.op_ltstar, // `<*>`
    $.op_stargt, // `<$>`
    $.op_ltbar, // `<|>`
    $.op_eqeq,
    $.op_slasheq,
    $.op_lt,
    $.op_le,
    $.op_gt,
    $.op_ge,
    $.op_andand,
    $.op_barbar,
    $.op_bind, // `>>=`

    // Raw captures: content the lexer keeps verbatim.
    $.prim_fragment, // `family { ... }` target-IR body text
    $.token_tree, // balanced `(...)`/`[...]`/`{...}` after `name!`
  ],

  // The scanner consumes whitespace and comments (it must see every
  // character for the minus rules); these extras only smooth error
  // recovery. Comments scan as hidden trivia tokens.
  extras: ($) => [/[ \t\n\r]/, $._comment],

  // `-> {` is an effect row in the parser (parser.rs
  // `parse_effect_row_opt`); the EBNF also lets a result type start
  // with an implicit binder `{x :: A} -> B`, so the two readings meet
  // here. Both are explored; only one ever parses.
  conflicts: ($) => [[$._type_application], [$._argument, $.record_construction], [$._argument, $.variable], [$.application], [$.list_literal, $.list_pattern], [$._atom, $.record_construction, $.record_pattern], [$._atom, $.constructor_pattern], [$._atom, $._atomic_pattern], [$.variable, $._atomic_pattern], [$._value_name, $._name], [$._atom, $.record_construction], [$._arrow_type], [$.capability_supply, $.map_literal], [$._annotation], [$._value_name, $._atomic_pattern], [$._unary_expr, $.negation_pattern], [$.variable, $.as_pattern], [$.constructor_path], [$.import_declaration], [$.arrow_head], [$._atom, $.variable]],

  rules: {
    // ----------------------------------------------------------- modules

    source_file: ($) => repeat($.module_declaration),

    module_declaration: ($) =>
      seq(
        optional($._attribute_list),
        $.kw_module,
        $.module_path,
        optional($.module_parameters),
        optional(seq($.colon_colon, $._type)),
        $.kw_where,
        $.declaration_block,
      ),

    module_path: ($) => seq($.constructor, repeat(seq($.dot, $.constructor))),

    module_parameters: ($) =>
      seq($.lparen, sepBy($.comma, $.module_parameter), $.rparen),

    module_parameter: ($) =>
      seq(choice($.identifier, $.constructor), $.colon_colon, $._type),

    // ------------------------------------------------------ declarations

    declaration_block: ($) => block($, $.declaration),

    declaration: ($) =>
      choice(
        $.import_declaration,
        $.import_data_declaration,
        $.export_declaration,
        $.signature_declaration,
        $.instance_declaration,
        $.data_declaration,
        $.record_declaration,
        $.codata_declaration,
        $.effect_declaration,
        $.opaque_type_declaration,
        $.generate_declaration,
        $.rewrite_declaration,
        $.prim_declaration,
        $.upgrade_declaration,
        $.contract_declaration,
        $.foreign_declaration,
        $.fixity_declaration,
        $.value_signature,
        $.equation,
      ),

    import_declaration: ($) =>
      seq(
        optional($._attribute_list),
        $.kw_import,
        optional(
          choice($.kw_stage, $.kw_replaceable, $.kw_specialized),
        ),
        $.module_path,
        optional($.import_clause),
        optional(seq($.kw_as, $.constructor)),
      ),

    // `import X (a as b, c)` selects members; `import M (args) as N`
    // instantiates module parameters. The two share one clause shape; the
    // distinction is the checker's (surface grammar local amendments
    // section 11, decisions D108/D115).
    import_clause: ($) =>
      seq($.lparen, sepBy($.comma, $.import_item), $.rparen),

    import_item: ($) =>
      choice(
        seq($._name, optional(seq($.kw_as, $._name))),
        $.int_literal,
      ),

    import_data_declaration: ($) =>
      seq(
        optional($._attribute_list),
        $.kw_import,
        $.kw_data,
        $.string_literal,
        $.kw_as,
        $.identifier,
      ),

    export_declaration: ($) =>
      seq(
        optional($._attribute_list),
        $.kw_export,
        sepBy1($.comma, $.export_item),
      ),

    export_item: ($) =>
      choice(
        seq($.kw_module, $.module_path),
        seq($._name, repeat1(seq($.dot, $._name))),
        $._name,
      ),

    signature_declaration: ($) =>
      seq(
        optional($._attribute_list),
        $.kw_signature,
        $.constructor,
        repeat($._parameter),
        $.kw_where,
        $.signature_block,
      ),

    signature_block: ($) => block($, $.signature_item),

    signature_item: ($) =>
      choice(
        $.opaque_type_declaration,
        $.record_declaration,
        $.law_declaration,
        $.method_signature,
      ),

    law_declaration: ($) =>
      seq(
        optional($._attribute_list),
        $.kw_law,
        $._value_name,
        $.colon_colon,
        $._type,
      ),

    method_signature: ($) =>
      seq(
        optional($._attribute_list),
        $._value_name,
        $.colon_colon,
        $._type,
      ),

    instance_declaration: ($) =>
      seq(
        optional($._attribute_list),
        optional($.kw_implicit),
        $.kw_instance,
        $.constructor,
        repeat($._parameter),
        $.colon_colon,
        $._type,
        $.kw_where,
        $.instance_block,
      ),

    instance_block: ($) => block($, $.equation),

    data_declaration: ($) =>
      seq(
        optional($._attribute_list),
        $.kw_data,
        $.constructor,
        repeat($._parameter),
        optional(seq($.colon_colon, $._type)),
        $.kw_where,
        $.constructor_block,
      ),

    constructor_block: ($) => block($, $.constructor_declaration),

    constructor_declaration: ($) =>
      seq(
        optional($._attribute_list),
        $.constructor,
        $.colon_colon,
        $._type,
      ),

    record_declaration: ($) =>
      seq(
        optional($._attribute_list),
        $.kw_record,
        $.constructor,
        repeat($._parameter),
        optional(seq($.colon_colon, $._type)),
        $.kw_where,
        $.field_block,
      ),

    codata_declaration: ($) =>
      seq(
        optional($._attribute_list),
        $.kw_codata,
        $.constructor,
        repeat($._parameter),
        optional(seq($.colon_colon, $._type)),
        $.kw_where,
        $.field_block,
      ),

    field_block: ($) => block($, $.field_declaration),

    field_declaration: ($) =>
      seq(
        optional($._attribute_list),
        optional($.kw_erased),
        $._value_name,
        $.colon_colon,
        optional($._attribute_list),
        $._type,
      ),

    effect_declaration: ($) =>
      seq(
        optional($._attribute_list),
        $.kw_effect,
        $.constructor,
        repeat($._effect_parameter),
        $.kw_where,
        $.operation_block,
      ),

    _effect_parameter: ($) =>
      choice(
        $.identifier,
        seq(
          $.lparen,
          $._value_name,
          optional(seq($.colon_colon, $.constructor)),
          $.rparen,
        ),
      ),

    operation_block: ($) => block($, $.operation_declaration),

    operation_declaration: ($) =>
      seq($._value_name, $.colon_colon, $._type),

    opaque_type_declaration: ($) =>
      seq(
        optional($._attribute_list),
        $.kw_opaque,
        $.kw_type,
        $.constructor,
        repeat($._parameter),
        $.colon_colon,
        $._type,
      ),

    generate_declaration: ($) =>
      seq(
        optional($._attribute_list),
        $.kw_generate,
        $.kw_do,
        $.do_block,
      ),

    rewrite_declaration: ($) =>
      seq(
        optional($._attribute_list),
        optional($.kw_target),
        $.kw_rewrite,
        $._value_name,
        $.kw_forall,
        repeat1($._type_binder),
        $.dot,
        optional($.using_clause),
        $.kw_where,
        $.rewrite_block,
      ),

    rewrite_block: ($) => block($, $.rewrite_clause),

    rewrite_clause: ($) =>
      seq(
        choice($.kw_match, $.kw_replace, $.kw_require, $.kw_justify),
        $._term,
      ),

    // `prim name :: type where { target_body ... }` and the `unsafe prim`
    // sugar that marks every body (D206-D212, D221-D227).
    prim_declaration: ($) =>
      seq(
        optional($._attribute_list),
        optional($.kw_unsafe),
        $.kw_prim,
        $._value_name,
        $.colon_colon,
        $._type,
        $.kw_where,
        $.prim_body_block,
      ),

    prim_body_block: ($) => block($, $.prim_body),

    // `family { fragment } [when e] [justify e] [uses ...] [immarg ...]`
    // with the per-body `unsafe` prefix.
    // The clauses come in any order, each at most once (parser.rs
    // `parse_prim_body`).
    prim_body: ($) =>
      seq(
        optional($.kw_unsafe),
        $.target_family,
        $.prim_fragment,
        repeat(
          choice(
            seq($.kw_when, $._term),
            seq($.kw_justify, $._term),
            $.prim_uses,
            $.prim_immarg,
          ),
        ),
      ),

    target_family: ($) =>
      choice($.kw_llvm, $.kw_wasm, $.kw_asm, $.kw_clr, $.kw_jvm),

    prim_uses: ($) =>
      seq(
        $.kw_uses,
        sepBy1(
          $.comma,
          seq($.lparen, $._value_name, $.colon_colon, $._type, $.rparen),
        ),
      ),

    prim_immarg: ($) =>
      seq($.kw_immarg, sepBy1($.comma, $.int_literal)),

    upgrade_declaration: ($) =>
      seq(
        optional($._attribute_list),
        $.kw_upgrade,
        $.module_path,
        $.kw_to,
        $.module_path,
        $.kw_where,
        $.assignment_block,
      ),

    assignment_block: ($) => block($, $.assignment),

    assignment: ($) => seq($._name, $.equals, $._term),

    // `contract <kind> <name> for <def> where { <op> = (P); assume = ... }`
    // (core language specification section 10; D198-D205).
    contract_declaration: ($) =>
      seq(
        optional($._attribute_list),
        $.kw_contract,
        $.contract_kind,
        $._value_name,
        $.kw_for,
        $._value_name,
        $.kw_where,
        $.contract_clause_block,
      ),

    contract_kind: ($) =>
      choice($.kw_safety, $.kw_liveness, $.kw_recurrence, $.kw_stability),

    contract_clause_block: ($) => block($, $.contract_clause),

    contract_clause: ($) =>
      choice(
        seq(
          choice($.kw_always, $.kw_eventually, $.kw_recur, $.kw_stable),
          $.equals,
          $.parenthesized_term,
        ),
        seq(
          $.kw_assume,
          $.equals,
          choice($.parenthesized_term, $.string_literal),
        ),
      ),

    parenthesized_term: ($) => seq($.lparen, $._term, $.rparen),

    foreign_declaration: ($) =>
      seq(
        optional($._attribute_list),
        $.kw_foreign,
        choice($.kw_import, $.kw_export),
        choice($.kw_ccall, $.kw_component),
        $.string_literal,
        $._value_name,
        $.colon_colon,
        $._type,
      ),

    fixity_declaration: ($) =>
      seq(
        optional($._attribute_list),
        choice($.kw_infix, $.kw_infixl, $.kw_infixr),
        $.int_literal,
        $._operator_lexeme,
        repeat(seq($.comma, $._operator_lexeme)),
      ),

    value_signature: ($) =>
      seq(
        optional($._attribute_list),
        optional($._modifier),
        $._value_name,
        optional($.macro_attributes),
        $.colon_colon,
        $._type,
      ),

    // `macro name attributes (codec, table) :: ty` (attribute annotations
    // specification section 6).
    macro_attributes: ($) =>
      seq(
        $.kw_attributes,
        $.lparen,
        sepBy($.comma, choice($.identifier, $.constructor)),
        $.rparen,
      ),

    equation: ($) =>
      seq(
        optional($._attribute_list),
        optional($._modifier),
        $._value_name,
        optional($._attribute_list_semi),
        repeat($._atomic_pattern),
        $.equals,
        $._term,
        optional(seq($.kw_where, $.local_block)),
      ),

    _modifier: ($) =>
      choice($.kw_proof, $.kw_spec, $.kw_stage, $.kw_corec, $.kw_macro),

    _value_name: ($) =>
      choice($.identifier, seq($.lparen, $._operator_lexeme, $.rparen)),

    // ---------------------------------------------------------- local blocks

    local_block: ($) => block($, $.local_declaration),

    local_declaration: ($) =>
      choice(
        $.local_signature,
        $.equation,
        $.pattern_binding,
        $.memo_binding,
      ),

    local_signature: ($) =>
      seq($._value_name, $.colon_colon, $._type),

    pattern_binding: ($) => seq($._pattern, $.equals, $._term),

    // `~ name = term` (laziness and reclamation specification section 5;
    // amendments section 2).
    memo_binding: ($) =>
      seq($.tilde, choice($.identifier, $.wildcard), $.equals, $._term),

    // ------------------------------------------------------- attributes

    // `#[name]`, `#[name(args)]`, `#[name = literal]` (amendments section
    // 5). The layout pass inserts a virtual `;` before an attribute on a
    // new line inside an item; the swallow sites (equation clause names,
    // `if` branch heads, guard heads) consume those separators.
    _attribute: ($) =>
      seq(
        $.attr_open,
        $.attr_name,
        optional($.attr_args),
        $.rbracket,
      ),

    // The layout pass inserts a virtual `;` before an attribute or its
    // construct on a new line; those separators belong to the annotated
    // item (parser.rs `parse_attr_list` skips them).
    _attribute_list: ($) =>
      repeat1(seq($._attribute, repeat($._layout_semi))),

    _attribute_list_semi: ($) =>
      seq(
        repeat($._layout_semi),
        repeat1(seq($._attribute, repeat($._layout_semi))),
      ),

    // ------------------------------------------------------------ terms

    _term: ($) =>
      choice(
        $.lambda,
        $.let_expression,
        $.letrec_expression,
        $.if_expression,
        $.case_expression,
        $.do_expression,
        $.handle_expression,
        $.with_expression,
        $.forall_expression,
        $.sigma_expression,
        $.using_expression,
        $._annotation,
      ),

    // `e :: t` binds looser than every expression operator.
    _annotation: ($) =>
      choice(
        seq($._dollar_expr, $.colon_colon, $._type),
        $._dollar_expr,
      ),

    // `$` is reserved right-associative application below all ordinary
    // infix operators; its right side is a whole term.
    _dollar_expr: ($) =>
      prec.right(
        PREC.dollar,
        choice(
          seq($._generic_expr, $.dollar, $._term),
          $._generic_expr,
        ),
      ),

    // Operators outside the documented table: one flat layer above `$`.
    _generic_expr: ($) =>
      choice(
        $._or_expr,
        prec.left(
          PREC.generic,
          seq(
            $._or_expr,
            $.operator,
            $._or_expr,
            repeat(seq($.operator, $._or_expr)),
          ),
        ),
      ),

    // The documented fixity table (see PREC).
    _or_expr: ($) =>
      infixRight($, PREC.or, ['op_barbar'], $._bind_expr),
    _bind_expr: ($) =>
      infixLeft($, PREC.bind, ['op_bind'], $._and_expr),
    _and_expr: ($) =>
      infixRight($, PREC.and, ['op_andand'], $._cmp_expr),
    _cmp_expr: ($) =>
      infixNonassoc($, PREC.cmp, ['op_eqeq', 'op_slasheq', 'op_lt', 'op_le', 'op_gt', 'op_ge'], $._alt_expr),
    _alt_expr: ($) =>
      infixLeft($, PREC.alt, ['op_ltbar'], $._bor_expr),
    _bor_expr: ($) =>
      infixLeft($, PREC.bor, ['pipe', 'op_ltstar', 'op_stargt'], $._bxor_expr),
    _bxor_expr: ($) =>
      infixLeft($, PREC.bxor, ['op_caret'], $._band_expr),
    _band_expr: ($) =>
      infixLeft($, PREC.band, ['op_amp'], $._concat_expr),
    _concat_expr: ($) =>
      infixRight($, PREC.concat, ['op_ltgt'], $._shift_expr),
    _shift_expr: ($) =>
      infixLeft($, PREC.shift, ['op_ltlt', 'op_gtgt'], $._add_expr),
    _add_expr: ($) =>
      infixLeft($, PREC.add, ['op_plus', 'op_minus'], $._mul_expr),
    _mul_expr: ($) =>
      infixLeft($, PREC.mul, ['op_star', 'op_slash', 'op_percent'], $.application),

    // Prefix negation binds to one postfixed atomic term (`- f x` is
    // `App(Neg(f), [x])`; write `-(f x)` for the application result).
    _unary_expr: ($) =>
      prec(
        PREC.prefix,
        choice(
          seq(repeat1($.op_minus), $._postfix_expr),
          $._postfix_expr,
        ),
      ),

    _postfix_expr: ($) =>
      seq(
        $._atom,
        repeat(choice($.projection, $.type_argument, $.capability_supply)),
      ),

    projection: ($) => seq($.dot, $.identifier),
    type_argument: ($) => seq($.at, $._atom),
    capability_supply: ($) =>
      seq($.at, $.lbrace, sepBy($.comma, $.assignment), $.rbrace),
    macro_invocation: ($) => seq($.variable, $.bang, $.token_tree),

    // `prefix { postfix }`: application binds tighter than every infix
    // operator (EBNF `application`).
    application: ($) => seq($._unary_expr, repeat($._argument_expr)),

    _argument_expr: ($) =>
      seq(
        $._argument,
        repeat(choice($.projection, $.type_argument, $.capability_supply)),
      ),

    // EBNF `atomic_term`.
    _atom: ($) =>
      choice(
        $.variable,
        $.constructor_path,
        $.record_construction,
        $.int_literal,
        $.float_literal,
        $.string_literal,
        $.char_literal,
        $.symbol_literal,
        $.type_name_literal,
        $.wildcard,
        $.unit,
        $.tuple_expression,
        $.parenthesized_expression,
        $.list_literal,
        $.map_literal,
        $.quotation,
        $.quote_decls,
        $.splice,
        $.handler_expression,
        $.operator_value,
        $.macro_invocation,
      ),

    // Postfix application arguments (parser.rs `starts_postfix`): no map
    // literals, no record-less brace forms.
    _argument: ($) =>
      choice(
        $.variable,
        $.constructor_path,
        $.record_construction,
        $.int_literal,
        $.float_literal,
        $.string_literal,
        $.char_literal,
        $.symbol_literal,
        $.type_name_literal,
        $.wildcard,
        $.unit,
        $.tuple_expression,
        $.parenthesized_expression,
        $.list_literal,
        $.quotation,
        $.quote_decls,
        $.splice,
        $.handler_expression,
        $.operator_value,
        $.macro_invocation,
      ),

    variable: ($) =>
      choice($.identifier, seq($.constructor_path, $.dot, $.identifier)),

    constructor_path: ($) => seq($.constructor, repeat(seq($.dot, $.constructor))),

    record_construction: ($) =>
      seq(
        $.constructor_path,
        $.lbrace,
        sepBy($.comma, $.assignment),
        $.rbrace,
      ),

    unit: ($) => seq($.lparen, $.rparen),

    tuple_expression: ($) =>
      seq($.lparen, $._term, repeat1(seq($.comma, $._term)), $.rparen),

    parenthesized_expression: ($) => seq($.lparen, $._term, $.rparen),

    operator_value: ($) => seq($.lparen, $._operator_lexeme, $.rparen),

    list_literal: ($) =>
      seq($.lbracket, sepBy($.comma, $._term), $.rbracket),

    // `{k => v, ...}` desugars to `from-map` (amendments section 10).
    map_literal: ($) =>
      seq(
        $.lbrace,
        sepBy($.comma, seq($._term, $.darrow, $._term)),
        $.rbrace,
      ),

    // `[| e |]` typed quotation (draft 0.2).
    quotation: ($) => seq($.lquote, $._term, $.rquote),

    // `quoteDecls where { ... }`.
    quote_decls: ($) =>
      seq($.kw_quotedecls, $.kw_where, $.declaration_block),

    // `$(code)`; the scanner emits `splice_open` only for a tight `$(`.
    splice: ($) => seq($.splice_open, $._term, $.rparen),

    // `handler L [r] where { return p -> e; op pats [resume k] -> e }`.
    handler_expression: ($) =>
      seq(
        $.kw_handler,
        $.constructor_path,
        optional($.identifier),
        $.kw_where,
        $.handler_block,
      ),

    handler_block: ($) => block($, $.handler_clause),

    handler_clause: ($) =>
      choice(
        seq($.kw_return, $._pattern, $.arrow, $._term),
        seq(
          $._value_name,
          repeat($._atomic_pattern),
          optional(seq($.kw_resume, $.identifier)),
          $.arrow,
          $._term,
        ),
      ),

    lambda: ($) =>
      seq(
        $.backslash,
        repeat1($._atomic_pattern),
        $.arrow,
        $._term,
      ),

    let_expression: ($) =>
      seq($.kw_let, $.local_block, $.kw_in, $._term),

    // `letrec` admits guarded recursion only (D103/D131-D133).
    letrec_expression: ($) =>
      seq($.kw_letrec, $.local_block, $.kw_in, $._term),

    if_expression: ($) =>
      seq(
        $.kw_if,
        optional($._attribute_list_semi),
        $._term,
        $.kw_then,
        optional($._attribute_list_semi),
        $._term,
        $.kw_else,
        optional($._attribute_list_semi),
        $._term,
      ),

    case_expression: ($) =>
      seq($.kw_case, $._term, $.kw_of, $.alternative_block),

    alternative_block: ($) => block($, $.alternative),

    alternative: ($) =>
      seq(
        optional($._attribute_list),
        $._pattern,
        optional(
          seq(
            $.pipe,
            optional($._attribute_list_semi),
            $._term,
          ),
        ),
        $.arrow,
        $._term,
      ),

    do_expression: ($) => seq($.kw_do, $.do_block),

    do_block: ($) => block($, $.do_statement),

    do_statement: ($) =>
      choice(
        seq($.kw_let, $.local_block),
        seq($._pattern, $.larrow, $._term),
        $._term,
      ),

    handle_expression: ($) =>
      seq($.kw_handle, $._term, $.kw_with, $._term),

    // `with (x = e) in body` (amendments section 7; capability supply).
    with_expression: ($) =>
      seq(
        $.kw_with,
        $.lparen,
        sepBy1($.comma, $.assignment),
        $.rparen,
        $.kw_in,
        $._term,
      ),

    forall_expression: ($) =>
      seq(
        $.kw_forall,
        repeat1($._type_binder),
        $.dot,
        $._term,
      ),

    sigma_expression: ($) =>
      seq($.kw_sigma, $.typed_binder, $.dot, $._term),

    using_expression: ($) =>
      seq($.using_clause, $.darrow, $._term),

    using_clause: ($) =>
      seq(
        $.kw_using,
        $.lparen,
        sepBy1($.comma, $.constraint),
        $.rparen,
      ),

    constraint: ($) => seq($.identifier, $.colon_colon, $._type_application),

    // ---------------------------------------------------------- patterns

    _pattern: ($) =>
      seq(
        choice($.constructor_pattern, $._atomic_pattern),
        optional(seq($.colon_colon, $._type)),
      ),

    constructor_pattern: ($) =>
      seq($.constructor_path, repeat1($._atomic_pattern)),

    _atomic_pattern: ($) =>
      choice(
        $.wildcard,
        $.identifier,
        $.constructor_path,
        $.record_pattern,
        $.int_literal,
        $.float_literal,
        $.string_literal,
        $.char_literal,
        $.symbol_literal,
        $.negation_pattern,
        $.unit,
        $.tuple_pattern,
        $.parenthesized_pattern,
        $.list_pattern,
        $.as_pattern,
      ),

    negation_pattern: ($) => seq($.op_minus, $._atomic_pattern),

    as_pattern: ($) => seq($.identifier, $.at, $._atomic_pattern),

    record_pattern: ($) =>
      seq(
        $.constructor_path,
        $.lbrace,
        sepBy1($.comma, seq($._value_name, $.equals, $._atom)),
        $.rbrace,
      ),

    tuple_pattern: ($) =>
      seq(
        $.lparen,
        $._pattern,
        repeat1(seq($.comma, $._pattern)),
        $.rparen,
      ),

    parenthesized_pattern: ($) => seq($.lparen, $._pattern, $.rparen),

    list_pattern: ($) =>
      seq($.lbracket, sepBy($.comma, $._pattern), $.rbracket),

    // ------------------------------------------------------------ types

    _type: ($) =>
      choice(
        $.forall_type,
        $.sigma_type,
        $.using_type,
        $._arrow_type,
      ),

    forall_type: ($) =>
      seq($.kw_forall, repeat1($._type_binder), $.dot, $._type),

    sigma_type: ($) => seq($.kw_sigma, $.typed_binder, $.dot, $._type),

    using_type: ($) => seq($.using_clause, $.darrow, $._type),

    _arrow_type: ($) =>
      choice(
        seq(
          $.typed_binder,
          optional($._multiplicity),
          $.arrow_head,
          $._type,
        ),
        seq(
          $.implicit_binder,
          optional($._multiplicity),
          $.arrow_head,
          $._type,
        ),
        seq(
          $._type_infix,
          optional(seq(optional($._multiplicity), $.arrow_head, $._type)),
        ),
      ),

    // `->` plus its optional effect row (`a -> b`, `a ->{IO r} b`,
    // `a ->{|e} b`).
    arrow_head: ($) => seq($.arrow, optional($.effect_row)),

    _multiplicity: ($) =>
      choice($.mult_zero, $.mult_one, $.mult_aff, $.mult_many),

    typed_binder: ($) =>
      seq(
        $.lparen,
        optional(choice($.kw_erased, $.kw_own)),
        $.identifier,
        $.colon_colon,
        $._type,
        $.rparen,
      ),

    implicit_binder: ($) =>
      seq(
        $.lbrace,
        optional($.kw_erased),
        $.identifier,
        optional(seq($.colon_colon, $._type)),
        $.rbrace,
      ),

    _type_binder: ($) =>
      choice(
        $.typed_binder,
        $.implicit_binder,
        $.identifier,
        $.constructor,
      ),

    // Type-level infix chains mirror the term layers over the same
    // documented table; operands are type applications (`Vec (n + 1) a`).
    _type_infix: ($) =>
      choice(
        $.type_chain,
        $._type_or_expr,
      ),

    // Undeclared operators in types: flat, like `parse_infix_type`.
    type_chain: ($) =>
      prec.left(
        PREC.generic,
        seq(
          $._type_or_expr,
          $.operator,
          $._type_or_expr,
          repeat(seq($.operator, $._type_or_expr)),
        ),
      ),

    _type_or_expr: ($) =>
      infixRight($, PREC.or, ['op_barbar'], $._type_bind_expr),
    _type_bind_expr: ($) =>
      infixLeft($, PREC.bind, ['op_bind'], $._type_and_expr),
    _type_and_expr: ($) =>
      infixRight($, PREC.and, ['op_andand'], $._type_cmp_expr),
    _type_cmp_expr: ($) =>
      infixNonassoc($, PREC.cmp, ['op_eqeq', 'op_slasheq', 'op_lt', 'op_le', 'op_gt', 'op_ge'], $._type_alt_expr),
    _type_alt_expr: ($) =>
      infixLeft($, PREC.alt, ['op_ltbar'], $._type_bor_expr),
    _type_bor_expr: ($) =>
      infixLeft($, PREC.bor, ['pipe', 'op_ltstar', 'op_stargt'], $._type_bxor_expr),
    _type_bxor_expr: ($) =>
      infixLeft($, PREC.bxor, ['op_caret'], $._type_band_expr),
    _type_band_expr: ($) =>
      infixLeft($, PREC.band, ['op_amp'], $._type_concat_expr),
    _type_concat_expr: ($) =>
      infixRight($, PREC.concat, ['op_ltgt'], $._type_shift_expr),
    _type_shift_expr: ($) =>
      infixLeft($, PREC.shift, ['op_ltlt', 'op_gtgt'], $._type_add_expr),
    _type_add_expr: ($) =>
      infixLeft($, PREC.add, ['op_plus', 'op_minus'], $._type_mul_expr),
    _type_mul_expr: ($) =>
      infixLeft($, PREC.mul, ['op_star', 'op_slash', 'op_percent'], $._type_application),

    // `Vec n a`, `Thunk {e} a`, `Lazy {e} r a` (amendments section 1).
    _type_application: ($) =>
      seq($._type_atom, repeat(choice($._type_atom, $.effect_row))),

    _type_atom: ($) =>
      choice(
        $.type_variable,
        $.qualified_value,
        $.constructor_path,
        $.int_literal,
        $.unit,
        $.tuple_type,
        $.parenthesized_type,
        $.arrow_former,
        $.tuple_former,
        $.type_name_literal,
      ),

    type_variable: ($) => $.identifier,

    // A dotted value name in type position: `List.map` in
    // `Eq (List.map f xs) ys` is the index-level function reference of a
    // proposition's argument (parser.rs `parse_type_atom`).
    qualified_value: ($) => seq($.constructor_path, $.dot, $.identifier),

    tuple_type: ($) =>
      seq(
        $.lparen,
        $._type,
        repeat1(seq($.comma, $._type)),
        $.rparen,
      ),

    // Index-level expressions may be full terms in parens (the EBNF:
    // terms occur in types).
    parenthesized_type: ($) =>
      seq($.lparen, choice($.lambda, $._type), $.rparen),

    // `(->)` the pure total arrow family (D155).
    arrow_former: ($) => seq($.lparen, $.arrow, $.rparen),

    // `(,)`, `(,,)`, ... structural tuple formers (D155).
    tuple_former: ($) => seq($.lparen, repeat1($.comma), $.rparen),

    // `{IO r, Async r | e}`, `{|e}`, `{e}` (a lone identifier is the row
    // variable; amendments section 1).
    effect_row: ($) =>
      seq(
        $.lbrace,
        optional(
          choice(
            $.identifier,
            seq(
              optional(sepBy($.comma, $.effect_label)),
              optional(seq($.pipe, $.identifier)),
            ),
          ),
        ),
        $.rbrace,
      ),

    effect_label: ($) =>
      seq(
        $.constructor_path,
        optional(choice($.identifier, $.constructor)),
      ),

    // ------------------------------------------------------- operators

    _operator_lexeme: ($) =>
      choice(
        $.operator,
        $.pipe,
        $.op_star,
        $.op_slash,
        $.op_percent,
        $.op_plus,
        $.op_minus,
        $.op_ltlt,
        $.op_gtgt,
        $.op_amp,
        $.op_caret,
        $.op_ltgt,
        $.op_ltstar,
        $.op_stargt,
        $.op_ltbar,
        $.op_eqeq,
        $.op_slasheq,
        $.op_lt,
        $.op_le,
        $.op_gt,
        $.op_ge,
        $.op_andand,
        $.op_barbar,
        $.op_bind,
      ),

    // ----------------------------------------------------------- names

    _name: ($) =>
      choice(
        $.identifier,
        $.constructor,
        seq($.lparen, $._operator_lexeme, $.rparen),
      ),

    _parameter: ($) =>
      choice(
        $.identifier,
        $.constructor,
        $.typed_binder,
        $.implicit_binder,
      ),
  },
});

function sepBy(separator, rule) {
  return optional(sepBy1(separator, rule));
}

function sepBy1(separator, rule) {
  return seq(rule, repeat(seq(separator, rule)));
}
