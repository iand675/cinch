// Tree-sitter grammar for the Cinch Core text format.
//
// Normative authority: design/core-output.md section 10 ("Text grammar
// (normative)") plus the printing rules of section 4. Section 8.2 fixes the
// node vocabulary: one node type per expression form in the section 10
// table, plus `type`, `class`, `span`, and the header/unit structure.
//
// Layout. The format is line oriented and every expression is a
// parenthesised form, so the parse does not need a scanner: a child list is
// the run of children between a form's own parentheses. The two printing
// invariants that a scannerless grammar cannot check are recorded in
// tooling/README.md rather than faked here:
//
//   * children of a node sit two spaces deeper per level, and
//   * a node whose children fit in 100 columns prints on one line.
//
// Both describe whitespace, which this grammar treats as trivia, so a
// wrapped node and a one-line node parse to the same tree. That is the
// property the corpus pins down (see corpus/layout.txt).
//
// Keywords. Form keywords (`unit`, `body`, `end`, `case`, `app`, ...) are
// anonymous literal tokens: they carry no meaning beyond delimiting a node,
// the rule that names them already requires the right one, and a missing
// `end` or `body` surfaces as a MISSING node. Editors match them as strings
// in queries. `end`, `body`, `unit` and the annotation words are therefore
// anonymous; annotations themselves, class names, spans and every form are
// named nodes.
//
// Identifiers. Canonical names are dot separated segments; a segment starts
// with a letter, `_` or `$` and may contain dashes between word characters.
// This spelling is what keeps `a->b` lexing as name `->` name instead of a
// single name `a-`, and `forall a b.` lexing as two names plus the binder
// terminator `.` instead of a name `b.`. Anything outside the spelling is
// quoted with `'...'` by the printer, and quoted names are accepted
// verbatim here.

// One dot separated canonical name segment. Dashes never end a segment so
// the type arrow stays a separate token.
const NAME_SEGMENT = /[A-Za-z_$][A-Za-z0-9_$]*(?:-[A-Za-z0-9_$]+)*/;
const NAME =
  /[A-Za-z_$][A-Za-z0-9_$]*(?:-[A-Za-z0-9_$]+)*(?:\.[A-Za-z0-9_$]+(?:-[A-Za-z0-9_$]+)*)*/;

/// A parenthesised form: `(keyword child*)`.
function form(keyword, ...children) {
  return seq('(', keyword, ...children, ')');
}

module.exports = grammar({
  name: 'cinch_core',

  extras: ($) => [/\s/, $.comment],

  rules: {
    // file = header line* module-body
    source_file: ($) =>
      seq($.header, repeat($._header_line), repeat($.unit)),

    header: ($) => seq('core', $._version),

    _header_line: ($) =>
      choice($.module_line, $.target_line, $.source_line),

    module_line: ($) => seq('module', $.name),

    // The target line appears only for stages where the target matters
    // (`lowered`); earlier stages are target independent.
    target_line: ($) =>
      seq('target', $.target_triple, 'opt-level', $._digits),

    source_line: ($) => seq('source', 'sha256', ':', $._sha256),

    target_triple: ($) => token(/[A-Za-z0-9_]+(?:-[A-Za-z0-9_.]+)+/),

    // unit = "unit " name " :: " type ann* NL ... "  body " expr NL "end"
    unit: ($) =>
      seq(
        'unit',
        $.unit_name,
        '::',
        $.type,
        repeat($.annotation),
        'body',
        $._expr,
        'end',
      ),

    unit_name: ($) => $.name,

    name: ($) =>
      choice($._bare_name, $._quoted_name),

    _bare_name: ($) => token(NAME),

    _quoted_name: ($) => token(seq("'", /[^'\n]*/, "'")),

    // Annotations print in a fixed order (@span, @class, @captures,
    // @specialised, @rewrite, @prim, @resolved, @tailrec, @join, @stage),
    // each on the unit line or as an indented `@` line when long. The
    // grammar accepts them in any order and in either position; order and
    // uniqueness are the printer's and verifier's invariants.
    annotation: ($) =>
      seq(
        '@',
        choice(
          // `@class owned` is the spelled out form; `@owned` is the
          // printed one (section 4), optionally with the ownership counts.
          seq('class', $.class),
          seq($.class, optional($.ownership_counts)),
          seq('span', $.span),
          seq('params', $.params),
          seq(
            $.annotation_name,
            optional($.annotation_args),
            // `@specialised(<keys>) from List.map` carries a trailing
            // source unit after its argument list.
            optional(seq('from', $.name)),
          ),
        ),
      ),

    annotation_name: ($) => $.name,

    // Annotation arguments are captured verbatim between balanced
    // parentheses. The design note fixes the names and the order but not
    // each argument's internal grammar (`@specialised(<Nat, Int>) from
    // List.map`, `@rewrite(<rule> v<n>)`, `@resolved(<instance> at
    // <span>)`), so structure below the annotation name would be invented
    // rather than normative.
    annotation_args: ($) =>
      seq(
        '(',
        repeat(choice($._arg_token, $._quoted_text, $.annotation_args)),
        ')',
      ),

    _arg_token: ($) => token(/[^()\s"]+/),

    ownership_counts: ($) =>
      seq('(', $._digits, repeat(seq(',', $._digits)), ')'),

    span: ($) => seq($._span_file, ':', $._digits, ':', $._digits),

    _span_file: ($) => token(/[^:\s()]+/),

    params: ($) => seq($.param, repeat(seq(',', $.param))),

    param: ($) => seq($.name, '::', $.type, '@', $.class),

    class: ($) =>
      choice('owned', 'borrowed', 'lent', 'uncounted', 'immortal'),

    // Types print in surface like canonical form. `forall a b. T` binds the
    // whole rest; arrows associate right; application is juxtaposition.
    type: ($) =>
      choice(
        prec.right(seq('forall', repeat1($.name), '.', $.type)),
        prec.right(seq($._type_application, '->', $.type)),
        $._type_application,
      ),

    _type_application: ($) => seq($._type_atom, repeat($._type_atom)),

    _type_atom: ($) =>
      choice(
        $.name,
        $.rigid_var,
        $.meta_var,
        $.refinement_type,
        $.row_type,
        seq('(', $.type, ')'),
      ),

    // A rigid skolem prints `#a`, a metavariable `?12`.
    rigid_var: ($) => token(seq('#', NAME_SEGMENT)),

    meta_var: ($) => token(seq('?', /[0-9]+/)),

    // refine (x :: T) with p
    refinement_type: ($) =>
      seq('refine', '(', $.name, '::', $.type, ')', 'with', $._predicate),

    // A predicate is a term over names, numerals and nested parentheses.
    // It is parsed greedily up to the next delimiter the enclosing type
    // needs (`->`, `,`, `)`, `}`, `body`, `@`), so a predicate that ends
    // in a type application needs parentheses around the application.
    _predicate: ($) =>
      prec.right(seq($._predicate_atom, repeat($._predicate_rest))),

    _predicate_rest: ($) =>
      choice($._predicate_atom, seq($._predicate_op, $._predicate_atom)),

    _predicate_atom: ($) =>
      choice(
        $.name,
        $.rigid_var,
        $.meta_var,
        $._digits,
        seq('(', $._predicate, ')'),
      ),

    _predicate_op: ($) =>
      token(
        choice(
          '&&',
          '||',
          '==',
          '/=',
          '!=',
          '<=',
          '>=',
          '<',
          '>',
          '+',
          '-',
          '*',
          '/',
          '^',
        ),
      ),

    // {IO r, Diverge}: a row of labels, each a type. A row applied to its
    // result ({IO r, Diverge} a) is type application, so the result type is
    // not part of this rule.
    row_type: ($) =>
      seq('{', optional(seq($.type, repeat(seq(',', $.type)))), '}'),

    // One node type per expression form in section 10's table.
    _expr: ($) =>
      choice(
        $.int_lit,
        $.nat_lit,
        $.bytes_lit,
        $.string_lit,
        $.sym_lit,
        $.bool_lit,
        $.unit_lit,
        $.local_ref,
        $.global_ref,
        $.ctor_ref,
        $.prim_ref,
        $.foreign_ref,
        $.app,
        $.lam,
        $.dict_proj,
        $.proj,
        $.inst_dict,
        $.let,
        $.if,
        $.case,
        $.dup,
        $.drop,
        $.release,
        $.box_init,
        $.box_get,
        $.box_clear,
        $.ref_set,
        $.quote,
        $.splice,
        $.syntax_lit,
        $.demand,
      ),

    int_lit: ($) => form('int', optional('-'), $._digits),

    // Nat digits keep their exact decimal spelling, so they print as a
    // quoted decimal string rather than a machine word.
    nat_lit: ($) => form('nat', $._quoted_digits),

    bytes_lit: ($) => form('bytes', $._quoted_text, $._digits),

    string_lit: ($) => form('string', $._quoted_text, $._digits),

    sym_lit: ($) => form('sym', $.name),

    bool_lit: ($) => form('bool', choice('true', 'false')),

    unit_lit: ($) => form('unit'),

    local_ref: ($) => form('local', $.name),

    global_ref: ($) => form('global', $.name),

    ctor_ref: ($) => form('ctor', $.name, $._digits, $._digits),

    prim_ref: ($) => form('prim', $.name),

    foreign_ref: ($) => form('foreign', $._quoted_text),

    app: ($) => form('app', $._expr, repeat($._expr)),

    // (lam (params) body); the captured env prints as a @captures
    // annotation on the unit, not in the form.
    lam: ($) => form('lam', '(', optional($.params), ')', $._expr),

    dict_proj: ($) => form('dictproj', $._expr, $._digits, $.name),

    proj: ($) => form('proj', $._expr, $.name),

    inst_dict: ($) => form('instdict', $.name, repeat($._expr)),

    let: ($) =>
      form('let', $.name, '::', $.type, '@', $.class, $._expr, $._expr),

    if: ($) => form('if', $._expr, $._expr, $._expr),

    // (case scrutinee (alt*)); the empty alternative list is `()`.
    case: ($) => form('case', $._expr, '(', repeat($.alt), ')'),

    alt: ($) => seq('(', $.pat, $._expr, ')'),

    // pat = _ | (<var>) | (<CtorName> <pat>*) | <int>
    // The two parenthesised shapes are one production: a bare variable is
    // the zero-argument case and is told apart from a nullary constructor
    // by the verifier, which knows the constructor set.
    pat: ($) =>
      choice(
        '_',
        seq('(', $.name, repeat($.pat), ')'),
        seq(optional('-'), $._digits),
      ),

    dup: ($) => form('dup', $.name),

    drop: ($) => form('drop', $.name),

    // (release x (moved xs) (rest ys) body); either field list may be empty.
    release: ($) =>
      form(
        'release',
        $.name,
        '(',
        'moved',
        repeat($.name),
        ')',
        '(',
        'rest',
        repeat($.name),
        ')',
        $._expr,
      ),

    box_init: ($) => form('boxinit', $.name, $._expr),

    box_get: ($) => form('boxget', $._expr),

    box_clear: ($) => form('boxclear', $.name),

    ref_set: ($) => form('refset', $.name, $._expr),

    quote: ($) => form('quote', $._digits, $._expr),

    splice: ($) => form('splice', $._digits),

    syntax_lit: ($) => form('syntaxlit', $._digits),

    demand: ($) => form('demand', $._expr),

    comment: ($) => token(seq('--', /[^\n]*/)),

    _version: ($) => token(/[0-9]+/),

    _digits: ($) => token(/[0-9]+/),

    _sha256: ($) => token(/[0-9a-f]{64}/),

    // Escaped UTF-8 text: printable characters, `\\`, `\"`, `\n`, `\t` and
    // `\xNN` for bytes below 0x20.
    _quoted_text: ($) => token(/"(\\.|[^"\\\n])*"/),

    // Nat constants print as digits only: `(nat "123...")`.
    _quoted_digits: ($) => token(/"[0-9]+"/),
  },
});
