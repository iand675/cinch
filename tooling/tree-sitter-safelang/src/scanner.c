/*
 * SafeLang external scanner: a faithful port of compiler/src/lexer.rs
 * and compiler/src/layout.rs.
 *
 * Every terminal is produced here, because the lexical rules are
 * context sensitive in ways a regex lexer cannot express:
 *
 *   - dash identifiers versus minus (D102/D128): a `-` tight on both
 *     sides between identifier characters is identifier glue
 *     (`int-digits-into` is one name); digits never continue an
 *     identifier across a dash, so `x-1` and `1-2` subtract;
 *   - negative literals (D129): a `-` tight only to following digits is
 *     the sign of a negative literal unless the previous character
 *     could end an expression (`)`, `]`, `}`, `"`, `'` or an identifier
 *     character), in which case it is the binary minus; a signed
 *     literal never joins a symbol run (`=-3` is `=` applied to `-3`);
 *   - tick syntax (D123): tick, one character, tick is a `Char` literal
 *     and wins; `'foo` is a Sym literal; `''T` is a type Name;
 *   - symbol runs: maximal munch; `$` continues a run (`<$>`) but never
 *     starts one; comments win over symbol runs at token boundaries;
 *     `%0`/`%1`/`%aff`/`%many` are reserved multiplicities;
 *   - layout (layout.rs): `where`, `of`, `do`, `let`/`letrec` open
 *     implicit blocks; equal indentation separates items; lower
 *     indentation closes blocks; `in` closes the nearest implicit `let`
 *     block without crossing an explicit delimiter; the `in` directly
 *     after a `with (...)` clause belongs to the clause; explicit `(`,
 *     `[`, `{` shield enclosing layout.
 *
 * Structure: `scan_token` is a port of `scan_token` and friends and is
 * run in two modes. Mark mode is the real scan: `mark_end` sits at
 * every position where the token could legally end, so tentative
 * lookahead characters (a dash that is not glue, a dot that does not
 * start a float, a multiplicity spelling that fails) never join the
 * token. Mark-free mode is the layout lookahead: it classifies the
 * next real token and runs the layout machine for it before that token
 * is scanned, so the machine's virtual `{`, `;`, `}` tokens are emitted
 * first; everything it consumes is discarded past the previous token's
 * final mark.
 *
 * State is a fixed-size POD, serialized wholesale.
 */

#include "tree_sitter/parser.h"

#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

enum TokenType {
  COMMENT,

  LAYOUT_OPEN,
  LAYOUT_SEMI,
  LAYOUT_CLOSE,

  IDENTIFIER,
  CONSTRUCTOR,
  WILDCARD,

  KW_MODULE,
  KW_WHERE,
  KW_DATA,
  KW_RECORD,
  KW_CODATA,
  KW_EFFECT,
  KW_SIGNATURE,
  KW_INSTANCE,
  KW_IMPLICIT,
  KW_IMPORT,
  KW_AS,
  KW_TYPE,
  KW_OPAQUE,
  KW_FORALL,
  KW_SIGMA,
  KW_USING,
  KW_IN,
  KW_LET,
  KW_LETREC,
  KW_IF,
  KW_THEN,
  KW_ELSE,
  KW_CASE,
  KW_OF,
  KW_DO,
  KW_HANDLE,
  KW_WITH,
  KW_REWRITE,
  KW_PRIM,
  KW_LAW,
  KW_TARGET,
  KW_JUSTIFY,
  KW_MATCH,
  KW_REPLACE,
  KW_REQUIRE,
  KW_UPGRADE,
  KW_CONTRACT,
  KW_TO,
  KW_GENERATE,
  KW_QUOTEDECLS,
  KW_FOREIGN,
  KW_EXPORT,
  KW_CCALL,
  KW_COMPONENT,
  KW_PROOF,
  KW_SPEC,
  KW_STAGE,
  KW_COREC,
  KW_MACRO,
  KW_INFIX,
  KW_INFIXL,
  KW_INFIXR,
  KW_ERASED,
  KW_OWN,
  KW_RETURN,
  KW_RESUME,

  KW_HANDLER,
  KW_UNSAFE,
  KW_WHEN,
  KW_USES,
  KW_IMMARG,
  KW_FOR,
  KW_SAFETY,
  KW_LIVENESS,
  KW_RECURRENCE,
  KW_STABILITY,
  KW_ALWAYS,
  KW_EVENTUALLY,
  KW_RECUR,
  KW_STABLE,
  KW_ASSUME,
  KW_MIGRATE,
  KW_RELATED,
  KW_ATTRIBUTES,
  KW_REPLACEABLE,
  KW_SPECIALIZED,
  KW_LLVM,
  KW_WASM,
  KW_ASM,
  KW_CLR,
  KW_JVM,

  COLON_COLON,
  ARROW,
  LARROW,
  DARROW,
  LQUOTE,
  RQUOTE,
  SPLICE_OPEN,
  MULT_ZERO,
  MULT_ONE,
  MULT_AFF,
  MULT_MANY,
  BANG,
  AT,
  DOT,
  EQUALS,
  PIPE,
  DOLLAR,
  BACKSLASH,
  TILDE,

  LPAREN,
  RPAREN,
  LBRACKET,
  RBRACKET,
  LBRACE,
  RBRACE,
  COMMA,
  SEMICOLON,

  ATTR_OPEN,
  ATTR_NAME,
  ATTR_ARGS,

  INT_LITERAL,
  FLOAT_LITERAL,
  STRING_LITERAL,
  CHAR_LITERAL,
  SYMBOL_LITERAL,
  TYPE_NAME_LITERAL,

  OPERATOR,
  OP_STAR,
  OP_SLASH,
  OP_PERCENT,
  OP_PLUS,
  OP_MINUS,
  OP_LTLT,
  OP_GTGT,
  OP_AMP,
  OP_CARET,
  OP_LTGT,
  OP_LTSTAR,
  OP_STARGT,
  OP_LTBAR,
  OP_EQEQ,
  OP_SLASHEQ,
  OP_LT,
  OP_LE,
  OP_GT,
  OP_GE,
  OP_ANDAND,
  OP_BARBAR,
  OP_BIND,

  PRIM_FRAGMENT,
  TOKEN_TREE,

  SYMBOL_COUNT,
};

/* Layout-relevant token classes (independent of the terminal). */
enum TokenKind {
  KIND_EOF,
  KIND_WORD,
  KIND_COMMENT,
  KIND_LPAREN,
  KIND_RPAREN,
  KIND_LBRACKET,
  KIND_RBRACKET,
  KIND_LBRACE,
  KIND_RBRACE,
  KIND_SPLICE,
  KIND_LQUOTE,
  KIND_RQUOTE,
  KIND_ATTR_CLOSE,
  KIND_OTHER,
};

#define MAX_FRAMES 64
#define MAX_QUEUE 32
#define MAX_WORD 96

enum FrameKind {
  FRAME_PAREN,
  FRAME_BRACKET,
  FRAME_BRACE,
  FRAME_BLOCK,
  FRAME_BLOCK_LET,
};

typedef struct {
  uint8_t kind;
  uint16_t col; /* blocks: the block column; delims: the with-clause
                   flag (layout.rs `Delim(_, bool)`) */
} Frame;

typedef struct {
  Frame frames[MAX_FRAMES];
  uint32_t n_frames;

  uint8_t queue[MAX_QUEUE];
  uint32_t q_head;
  uint32_t q_len;

  bool pending_layout;
  bool pending_layout_is_let;

  bool with_paren_pending;
  bool supply_close_pending;

  bool prim_active;
  uint16_t prim_col;
  bool prim_seen_where;
  char last_word[MAX_WORD];

  bool bang_pending;
  uint8_t attr_state;

  uint32_t cur_line;
  uint32_t cur_col;
  bool line_leading;
  char prev_char;

  uint32_t prev_tok_line;
  bool prev_tok_valid;

  bool machine_done;
} Scanner;

static const char *SYM_NAMES[] = {
  "COMMENT","LAYOUT_OPEN","LAYOUT_SEMI","LAYOUT_CLOSE","IDENTIFIER","CONSTRUCTOR","WILDCARD",
  "KW_MODULE","KW_WHERE","KW_DATA","KW_RECORD","KW_CODATA","KW_EFFECT","KW_SIGNATURE","KW_INSTANCE",
  "KW_IMPLICIT","KW_IMPORT","KW_AS","KW_TYPE","KW_OPAQUE","KW_FORALL","KW_SIGMA","KW_USING","KW_IN",
  "KW_LET","KW_LETREC","KW_IF","KW_THEN","KW_ELSE","KW_CASE","KW_OF","KW_DO","KW_HANDLE","KW_WITH",
  "KW_REWRITE","KW_PRIM","KW_LAW","KW_TARGET","KW_JUSTIFY","KW_MATCH","KW_REPLACE","KW_REQUIRE",
  "KW_UPGRADE","KW_CONTRACT","KW_TO","KW_GENERATE","KW_QUOTEDECLS","KW_FOREIGN","KW_EXPORT","KW_CCALL",
  "KW_COMPONENT","KW_PROOF","KW_SPEC","KW_STAGE","KW_COREC","KW_MACRO","KW_INFIX","KW_INFIXL","KW_INFIXR",
  "KW_ERASED","KW_OWN","KW_RETURN","KW_RESUME",
  "KW_HANDLER","KW_UNSAFE","KW_WHEN","KW_USES","KW_IMMARG","KW_FOR","KW_SAFETY","KW_LIVENESS",
  "KW_RECURRENCE","KW_STABILITY","KW_ALWAYS","KW_EVENTUALLY","KW_RECUR","KW_STABLE","KW_ASSUME",
  "KW_MIGRATE","KW_RELATED","KW_ATTRIBUTES","KW_REPLACEABLE","KW_SPECIALIZED","KW_LLVM","KW_WASM",
  "KW_ASM","KW_CLR","KW_JVM",
  "COLON_COLON","ARROW","LARROW","DARROW","LQUOTE","RQUOTE","SPLICE_OPEN","MULT_ZERO","MULT_ONE",
  "MULT_AFF","MULT_MANY","BANG","AT","DOT","EQUALS","PIPE","DOLLAR","BACKSLASH","TILDE",
  "LPAREN","RPAREN","LBRACKET","RBRACKET","LBRACE","RBRACE","COMMA","SEMICOLON",
  "ATTR_OPEN","ATTR_NAME","ATTR_ARGS",
  "INT_LITERAL","FLOAT_LITERAL","STRING_LITERAL","CHAR_LITERAL","SYMBOL_LITERAL","TYPE_NAME_LITERAL",
  "OPERATOR","OP_STAR","OP_SLASH","OP_PERCENT","OP_PLUS","OP_MINUS","OP_LTLT","OP_GTGT","OP_AMP",
  "OP_CARET","OP_LTGT","OP_LTSTAR","OP_STARGT","OP_LTBAR","OP_EQEQ","OP_SLASHEQ","OP_LT","OP_LE",
  "OP_GT","OP_GE","OP_ANDAND","OP_BARBAR","OP_BIND",
  "PRIM_FRAGMENT","TOKEN_TREE",
};

typedef struct {
  bool on;
  char prev_char;
  uint32_t line;
  uint32_t col;
  bool leading;
} Marks;

static Marks marks;

/* Replay buffer: the scanner cannot rewind to a token's final mark, but
   the layout lookahead (and the contextual word decisions) must read
   from there. Everything consumed past the last mark lands here and is
   replayed first; the buffer is reset at every mark (the last mark is
   the only position ever rewound to). Overflow past the buffer is
   dropped: no token type consumes more than a handful of characters
   past its final mark (the tentative rejects), and every long capture
   marks at its end. */
#define MAX_REPLAY 64
static char replay[MAX_REPLAY];
static unsigned r_len, r_pos;

static void r_rewind(void) { r_pos = 0; }

/* The character at the scan position: replay first, then the lexer. */
static char peekc(TSLexer *lexer) {
  if (r_pos < r_len) return replay[r_pos];
  return (char)lexer->lookahead;
}

static bool at_eof(TSLexer *lexer) { return peekc(lexer) == 0; }

/* Mark the current position as a legal token end (the last mark wins;
   marks never move backwards). Everything consumed since the mark is
   tentative lookahead and is replayed rather than lost, so the buffer
   restarts at every mark: only the overflow past the final mark is
   ever rewound over. */
static void mark(Scanner *s, TSLexer *lexer) {
  if (!marks.on) return;
  lexer->mark_end(lexer);
  r_len = 0;
  r_pos = 0;
  marks.prev_char = s->prev_char;
  marks.line = s->cur_line;
  marks.col = s->cur_col;
  marks.leading = s->line_leading;
}

static void take(Scanner *s, TSLexer *lexer, bool skip) {
  char c;
  if (r_pos < r_len) {
    /* Replaying overflow past the mark: already consumed physically,
       so the lexer is not advanced again. */
    c = replay[r_pos++];
  } else {
    c = (char)lexer->lookahead;
    lexer->advance(lexer, skip);
    if (marks.on && r_len < MAX_REPLAY) replay[r_len++] = c;
    /* Normal scanning reads at the buffer end; only the layout
       lookahead rewinds `r_pos`. */
    r_pos = r_len;
  }
  if (c == '\n') {
    s->cur_line++;
    s->cur_col = 1;
    s->line_leading = true;
  } else {
    s->cur_col++;
    if (c != ' ' && c != '\t' && c != '\r') s->line_leading = false;
  }
  s->prev_char = c;
}

/* ------------------------------------------------------- lex classes */

static bool is_symbol_char(char c) {
  return c == '!' || c == '#' || c == '%' || c == '&' || c == '*' || c == '+' ||
         c == '/' || c == '<' || c == '=' || c == '>' || c == '?' || c == '@' ||
         c == '^' || c == '|' || c == '~' || c == ':' || c == '-';
}

static bool is_ident_char(char c) {
  return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') ||
         (c >= '0' && c <= '9') || c == '_' || c == '\'';
}

/* Digits do not continue an identifier across a dash (D128). */
static bool is_dash_glue_char(char c) {
  return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_' ||
         c == '\'';
}

static bool is_expr_end_char(char c) {
  return c == ')' || c == ']' || c == '}' || c == '"' || c == '\'';
}

static bool is_digit(char c) { return c >= '0' && c <= '9'; }

static bool is_hex_digit(char c) {
  return is_digit(c) || (c >= 'a' && c <= 'f') || (c >= 'A' && c <= 'F');
}

static bool is_alpha(char c) {
  return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z');
}

static bool is_word_start(char c) { return is_alpha(c) || c == '_'; }

/* D102/D129: a `-` tight only to following digits is a literal sign
   unless the previous character could end an expression. */
static bool sign_minus_context(char prev, char next) {
  if (!is_digit(next)) return false;
  if (prev == 0) return true;
  if (is_ident_char(prev) || is_expr_end_char(prev)) return false;
  return true;
}

/* ---------------------------------------------------------- keywords */

struct WordSym {
  const char *word;
  int symbol;
};

static const struct WordSym RESERVED_WORDS[] = {
    {"module", KW_MODULE},         {"where", KW_WHERE},
    {"data", KW_DATA},             {"record", KW_RECORD},
    {"codata", KW_CODATA},         {"effect", KW_EFFECT},
    {"signature", KW_SIGNATURE},   {"instance", KW_INSTANCE},
    {"implicit", KW_IMPLICIT},     {"import", KW_IMPORT},
    {"as", KW_AS},                 {"type", KW_TYPE},
    {"opaque", KW_OPAQUE},         {"forall", KW_FORALL},
    {"Sigma", KW_SIGMA},           {"using", KW_USING},
    {"in", KW_IN},                 {"let", KW_LET},
    {"letrec", KW_LETREC},         {"if", KW_IF},
    {"then", KW_THEN},             {"else", KW_ELSE},
    {"case", KW_CASE},             {"of", KW_OF},
    {"do", KW_DO},                 {"handle", KW_HANDLE},
    {"with", KW_WITH},             {"rewrite", KW_REWRITE},
    {"prim", KW_PRIM},             {"law", KW_LAW},
    {"target", KW_TARGET},         {"justify", KW_JUSTIFY},
    {"match", KW_MATCH},           {"replace", KW_REPLACE},
    {"require", KW_REQUIRE},       {"upgrade", KW_UPGRADE},
    {"contract", KW_CONTRACT},     {"to", KW_TO},
    {"generate", KW_GENERATE},     {"quoteDecls", KW_QUOTEDECLS},
    {"foreign", KW_FOREIGN},       {"export", KW_EXPORT},
    {"ccall", KW_CCALL},           {"component", KW_COMPONENT},
    {"proof", KW_PROOF},           {"spec", KW_SPEC},
    {"stage", KW_STAGE},           {"corec", KW_COREC},
    {"macro", KW_MACRO},           {"infix", KW_INFIX},
    {"infixl", KW_INFIXL},         {"infixr", KW_INFIXR},
    {"erased", KW_ERASED},         {"own", KW_OWN},
    {"return", KW_RETURN},         {"resume", KW_RESUME},
    {NULL, 0},
};

static const struct WordSym CONTEXTUAL_WORDS[] = {
    {"handler", KW_HANDLER},       {"unsafe", KW_UNSAFE},
    {"when", KW_WHEN},             {"uses", KW_USES},
    {"immarg", KW_IMMARG},         {"for", KW_FOR},
    {"safety", KW_SAFETY},         {"liveness", KW_LIVENESS},
    {"recurrence", KW_RECURRENCE}, {"stability", KW_STABILITY},
    {"always", KW_ALWAYS},         {"eventually", KW_EVENTUALLY},
    {"recur", KW_RECUR},           {"stable", KW_STABLE},
    {"assume", KW_ASSUME},         {"migrate", KW_MIGRATE},
    {"related", KW_RELATED},       {"attributes", KW_ATTRIBUTES},
    {"replaceable", KW_REPLACEABLE},
    {"specialized", KW_SPECIALIZED},
    {"llvm", KW_LLVM},             {"wasm", KW_WASM},
    {"asm", KW_ASM},               {"clr", KW_CLR},
    {"jvm", KW_JVM},
    {NULL, 0},
};

static int lookup_word(const struct WordSym *table, const char *w) {
  for (const struct WordSym *p = table; p->word; p++) {
    if (strcmp(p->word, w) == 0) return p->symbol;
  }
  return -1;
}

static bool is_reserved_word(const char *w) {
  return lookup_word(RESERVED_WORDS, w) >= 0;
}

static bool is_family_word(const char *w) {
  return strcmp(w, "llvm") == 0 || strcmp(w, "wasm") == 0 ||
         strcmp(w, "asm") == 0 || strcmp(w, "clr") == 0 ||
         strcmp(w, "jvm") == 0;
}

/* Symbol runs: reserved punctuation (lexer.rs `lex_symbol`), then the
   documented fixity table; anything else is a plain `operator`. */
static int operator_symbol(const char *run) {
  static const struct WordSym OPS[] = {
      {"::", COLON_COLON},   {"->", ARROW},      {"<-", LARROW},
      {"=>", DARROW},        {"!", BANG},        {"@", AT},
      {"=", EQUALS},         {"|", PIPE},        {"~", TILDE},
      {"*", OP_STAR},        {"/", OP_SLASH},    {"%", OP_PERCENT},
      {"+", OP_PLUS},        {"-", OP_MINUS},    {"<<", OP_LTLT},
      {">>", OP_GTGT},       {"&", OP_AMP},      {"^", OP_CARET},
      {"<>", OP_LTGT},       {"<*>", OP_LTSTAR}, {"<$>", OP_STARGT},
      {"<|>", OP_LTBAR},     {"==", OP_EQEQ},    {"/=", OP_SLASHEQ},
      {"<", OP_LT},          {"<=", OP_LE},      {">", OP_GT},
      {">=", OP_GE},         {"&&", OP_ANDAND},  {"||", OP_BARBAR},
      {">>=", OP_BIND},      {NULL, 0},
  };
  return lookup_word(OPS, run);
}

/* ------------------------------------------------------------- state */

void *tree_sitter_safelang_external_scanner_create(void) {
  Scanner *s = (Scanner *)calloc(1, sizeof(Scanner));
  s->cur_line = 1;
  s->cur_col = 1;
  s->line_leading = true;
  return s;
}

void tree_sitter_safelang_external_scanner_destroy(void *payload) {
  free(payload);
}

unsigned tree_sitter_safelang_external_scanner_serialize(void *payload,
                                                        char *buffer) {
  Scanner *s = (Scanner *)payload;
  if (sizeof(Scanner) > TREE_SITTER_SERIALIZATION_BUFFER_SIZE) return 0;
  memcpy(buffer, s, sizeof(Scanner));
  return (unsigned)sizeof(Scanner);
}

void tree_sitter_safelang_external_scanner_deserialize(void *payload,
                                                      const char *buffer,
                                                      unsigned length) {
  Scanner *s = (Scanner *)payload;
  memset(s, 0, sizeof(Scanner));
  s->cur_line = 1;
  s->cur_col = 1;
  s->line_leading = true;
  if (length == sizeof(Scanner)) memcpy(s, buffer, sizeof(Scanner));
}

/* ---------------------------------------------------------- scanning */

typedef struct {
  int kind;   /* enum TokenKind */
  int symbol; /* enum TokenType; -1 for words (chosen after the scan) */
  char word[MAX_WORD];
} TokenClass;

static void queue_push(Scanner *s, uint8_t sym) {
  if (s->q_len < MAX_QUEUE) s->queue[s->q_len++] = sym;
}

/* Continue a symbol run from the current position (`lex_symbol`); `run`
   holds the characters consumed so far. */
static void scan_run(Scanner *s, TSLexer *lexer, char *run, size_t *rlen) {
  for (;;) {
    char c = peekc(lexer);
    if (c == '$') {
      /* `$` continues a run but never starts one; `$(` is a splice. */
      take(s, lexer, false);
      if (peekc(lexer) == '(') break; /* cut before the `$` */
      if (*rlen + 1 < MAX_WORD) run[(*rlen)++] = '$';
      mark(s, lexer);
      continue;
    }
    if (!is_symbol_char(c)) break;
    if (c == '-') {
      /* A negative-literal sign never joins a run (`=-3`). */
      char prev = s->prev_char;
      take(s, lexer, false);
      if (sign_minus_context(prev, peekc(lexer))) break;
      if (*rlen + 1 < MAX_WORD) run[(*rlen)++] = '-';
      mark(s, lexer);
      continue;
    }
    if (*rlen + 1 < MAX_WORD) run[(*rlen)++] = c;
    take(s, lexer, false);
    mark(s, lexer);
  }
  run[*rlen] = 0;
}

/* `lex_number`; `.` starts a float only when a digit follows, and the
   number may end before it (`1.foo`). */
static int scan_number(Scanner *s, TSLexer *lexer) {
  bool is_float = false;
  if (peekc(lexer) == '0') {
    take(s, lexer, false);
    mark(s, lexer);
    char n1 = peekc(lexer);
    if (n1 == 'x' || n1 == 'X') {
      take(s, lexer, false);
      unsigned digits = 0;
      for (;;) {
        char d = peekc(lexer);
        if (is_hex_digit(d)) {
          digits++;
          take(s, lexer, false);
          mark(s, lexer);
        } else if (d == '_') {
          take(s, lexer, false);
        } else {
          break;
        }
      }
      if (digits == 0) return -1;
      return INT_LITERAL;
    }
    while (is_digit(peekc(lexer)) ||
           peekc(lexer) == '_') {
      take(s, lexer, false);
      mark(s, lexer);
    }
  } else {
    while (is_digit(peekc(lexer)) ||
           peekc(lexer) == '_') {
      take(s, lexer, false);
      mark(s, lexer);
    }
  }
  if (peekc(lexer) == '.') {
    take(s, lexer, false); /* cut before the dot stays marked */
    if (is_digit(peekc(lexer))) {
      is_float = true;
      while (is_digit(peekc(lexer)) ||
             peekc(lexer) == '_') {
        take(s, lexer, false);
        mark(s, lexer);
      }
      if (peekc(lexer) == 'e' || peekc(lexer) == 'E') {
        take(s, lexer, false);
        if (peekc(lexer) == '+' ||
            peekc(lexer) == '-') {
          take(s, lexer, false);
        }
        while (is_digit(peekc(lexer))) {
          take(s, lexer, false);
          mark(s, lexer);
        }
      }
    }
  }
  return is_float ? FLOAT_LITERAL : INT_LITERAL;
}

/* `lex_escape` after the backslash. */
static bool scan_escape(Scanner *s, TSLexer *lexer) {
  if (at_eof(lexer)) return false;
  char c = peekc(lexer);
  take(s, lexer, false);
  switch (c) {
  case '\\':
  case '"':
  case '\'':
  case 'n':
  case 'r':
  case 't':
    return true;
  case 'u':
    if (peekc(lexer) != '{') return false;
    take(s, lexer, false);
    for (;;) {
      if (at_eof(lexer)) return false;
      char h = peekc(lexer);
      if (h == '}') {
        take(s, lexer, false);
        return true;
      }
      if (!is_hex_digit(h)) return false;
      take(s, lexer, false);
    }
  default:
    return false;
  }
}

static bool scan_string(Scanner *s, TSLexer *lexer) {
  take(s, lexer, false); /* the opening quote */
  for (;;) {
    if (at_eof(lexer)) return false;
    char c = peekc(lexer);
    if (c == '\n' || c == '\r') return false;
    if (c == '"') {
      take(s, lexer, false);
      mark(s, lexer);
      return true;
    }
    if (c == '\\') {
      take(s, lexer, false);
      if (!scan_escape(s, lexer)) return false;
    } else {
      take(s, lexer, false);
    }
    mark(s, lexer);
  }
}

/* `lex_word` from the current position: identifier characters plus dash
   glue. */
static void scan_word_rest(Scanner *s, TSLexer *lexer, char *word,
                           size_t *len) {
  for (;;) {
    char c = peekc(lexer);
    if (is_ident_char(c)) {
      if (*len + 1 < MAX_WORD) word[(*len)++] = c;
      take(s, lexer, false);
      mark(s, lexer);
    } else if (c == '-') {
      take(s, lexer, false); /* cut before the dash stays marked */
      if (is_dash_glue_char(peekc(lexer))) {
        if (*len + 1 < MAX_WORD) word[(*len)++] = '-';
      } else {
        return;
      }
    } else {
      return;
    }
  }
}

/* The balanced raw capture of a `prim` fragment (`lex_fragment`); the
   opening `{` is consumed. */
static bool scan_fragment(Scanner *s, TSLexer *lexer) {
  unsigned depth = 1;
  bool in_str = false;
  while (!at_eof(lexer)) {
    char c = peekc(lexer);
    if (in_str) {
      if (c == '\\') {
        take(s, lexer, false);
        if (!at_eof(lexer)) take(s, lexer, false);
      } else if (c == '"') {
        in_str = false;
        take(s, lexer, false);
      } else if (c == '\n') {
        return false;
      } else {
        take(s, lexer, false);
      }
      continue;
    }
    if (c == '"') {
      in_str = true;
      take(s, lexer, false);
    } else if (c == '{') {
      depth++;
      take(s, lexer, false);
    } else if (c == '}') {
      depth--;
      take(s, lexer, false);
      if (depth == 0) {
        mark(s, lexer);
        return true;
      }
    } else {
      take(s, lexer, false);
    }
  }
  return false;
}

/* The balanced raw capture of a macro token tree (`name!(...)`); the
   opening delimiter is consumed and `depth` starts at one. Comments
   are trivia inside (the compiler tokenizes there). */
static bool scan_token_tree(Scanner *s, TSLexer *lexer, unsigned depth,
                            char close) {
  bool in_str = false;
  while (!at_eof(lexer)) {
    char c = peekc(lexer);
    if (in_str) {
      if (c == '\\') {
        take(s, lexer, false);
        if (!at_eof(lexer)) take(s, lexer, false);
      } else if (c == '"') {
        in_str = false;
        take(s, lexer, false);
      } else {
        take(s, lexer, false);
      }
      continue;
    }
    if (c == '"') {
      in_str = true;
      take(s, lexer, false);
      continue;
    }
    if (c == '-') {
      take(s, lexer, false);
      if (peekc(lexer) == '-') {
        while (peekc(lexer) != '\n' && !at_eof(lexer))
          take(s, lexer, false);
        continue;
      }
      continue;
    }
    if (c == '{') {
      take(s, lexer, false);
      if (peekc(lexer) == '-') {
        take(s, lexer, false);
        unsigned d2 = 1;
        while (d2 > 0 && !at_eof(lexer)) {
          char e = peekc(lexer);
          if (e == '{') {
            take(s, lexer, false);
            if (peekc(lexer) == '-') {
              take(s, lexer, false);
              d2++;
            }
          } else if (e == '-') {
            take(s, lexer, false);
            if (peekc(lexer) == '}') {
              take(s, lexer, false);
              d2--;
            }
          } else {
            take(s, lexer, false);
          }
        }
        continue;
      }
      depth++;
      continue;
    }
    if (c == '(' || c == '[') {
      depth++;
      take(s, lexer, false);
      continue;
    }
    if (c == ')' || c == ']' || c == '}') {
      depth--;
      take(s, lexer, false);
      if (depth == 0) {
        if (c != close) return false;
        mark(s, lexer);
        return true;
      }
      continue;
    }
    take(s, lexer, false);
  }
  return false;
}

/* Raw attribute arguments (`lex_attr`): `( ... )` with opaque strings,
   or the `= literal` tail up to `]`. */
static bool scan_attr_args(Scanner *s, TSLexer *lexer) {
  if (peekc(lexer) == '(') {
    take(s, lexer, false);
    unsigned depth = 1;
    bool in_str = false;
    while (!at_eof(lexer)) {
      char d = peekc(lexer);
      if (in_str) {
        if (d == '\\') {
          take(s, lexer, false);
          if (!at_eof(lexer)) take(s, lexer, false);
        } else if (d == '"') {
          in_str = false;
          take(s, lexer, false);
        } else {
          take(s, lexer, false);
        }
        continue;
      }
      if (d == '"') {
        in_str = true;
        take(s, lexer, false);
      } else if (d == '(') {
        depth++;
        take(s, lexer, false);
      } else if (d == ')') {
        depth--;
        take(s, lexer, false);
        if (depth == 0) {
          mark(s, lexer);
          return true;
        }
      } else {
        take(s, lexer, false);
      }
    }
    return false;
  }
  if (peekc(lexer) == '=') {
    /* At least the `=` is consumed and marked, so the token is never
       zero width. */
    if (at_eof(lexer)) return false;
    while (peekc(lexer) != ']' && !at_eof(lexer)) {
      take(s, lexer, false);
      mark(s, lexer);
    }
    return true;
  }
  return false;
}

static bool scan_attr_name(Scanner *s, TSLexer *lexer) {
  unsigned n = 0;
  for (;;) {
    char c = peekc(lexer);
    if (is_ident_char(c) || c == '.' || c == '-') {
      n++;
      take(s, lexer, false);
      mark(s, lexer);
    } else {
      break;
    }
  }
  return n > 0;
}

/* `lex_multiplicity` plus the rest of the run when the spelling does
   not match (`%affix` is `%` then `affix`; `%+` is the run `%+`). */
static void scan_percent(Scanner *s, TSLexer *lexer, char *run,
                         size_t *rlen) {
  run[(*rlen)++] = '%';
  take(s, lexer, false);
  mark(s, lexer);
  char n1 = peekc(lexer);
  int mult = -1;
  bool consumed = false;
  if (n1 == '0' || n1 == '1') {
    char save = n1;
    take(s, lexer, false);
    consumed = true;
    if (!is_ident_char(peekc(lexer))) {
      mult = save == '0' ? MULT_ZERO : MULT_ONE;
    }
  } else if (n1 == 'a') {
    take(s, lexer, false);
    consumed = true;
    if (peekc(lexer) == 'f') {
      take(s, lexer, false);
      if (peekc(lexer) == 'f') {
        take(s, lexer, false);
        if (!is_ident_char(peekc(lexer))) mult = MULT_AFF;
      }
    }
  } else if (n1 == 'm') {
    take(s, lexer, false);
    consumed = true;
    if (peekc(lexer) == 'a') {
      take(s, lexer, false);
      if (peekc(lexer) == 'n') {
        take(s, lexer, false);
        if (peekc(lexer) == 'y') {
          take(s, lexer, false);
          if (!is_ident_char(peekc(lexer))) mult = MULT_MANY;
        }
      }
    }
  }
  if (mult >= 0) {
    mark(s, lexer);
    run[*rlen] = 0;
    /* Encode the multiplicity in the run text for the caller. */
    run[0] = (char)(mult == MULT_ZERO ? 1 : mult == MULT_ONE ? 2
                                            : mult == MULT_AFF ? 3
                                                               : 4);
    run[*rlen = 1] = 0;
    return;
  }
  if (consumed) {
    /* The failed spelling characters are not symbol characters: the
       token is the bare `%` and they are re-lexed as a word. */
    run[*rlen] = 0;
    return;
  }
  scan_run(s, lexer, run, rlen);
}

/* Keyword extraction is ours (every terminal is external). `next` is
   the classification of the following real token (or NULL at end of
   input); the D221-D227 `unsafe` prefix and the macro `attributes`
   list are decided with it. */
static int word_symbol(const bool *valid, const char *word,
                       const TokenClass *next) {
  int r = lookup_word(RESERVED_WORDS, word);
  if (r >= 0) return r;
  if (strcmp(word, "_") == 0) return WILDCARD;
  bool is_con = word[0] >= 'A' && word[0] <= 'Z';
  int ctx = lookup_word(CONTEXTUAL_WORDS, word);
  if (ctx >= 0) {
    switch (ctx) {
    case KW_HANDLER:
      /* parser.rs `parse_atom`: an atom-position `handler` is the
         handler form; prefer it where the parser accepts it. */
      if (valid[KW_HANDLER]) return KW_HANDLER;
      break;
    case KW_UNSAFE:
      /* D221-D227: contextual so `unsafe` stays an identifier; the
         prefix only appears before `prim` or a registered family. */
      if (valid[KW_UNSAFE] && next && next->kind == KIND_WORD &&
          (strcmp(next->word, "prim") == 0 || is_family_word(next->word))) {
        return KW_UNSAFE;
      }
      break;
    case KW_ATTRIBUTES:
      /* `macro name attributes (...)`: only before `(`. */
      if (valid[KW_ATTRIBUTES] && next && next->kind == KIND_LPAREN)
        return KW_ATTRIBUTES;
      break;
    default:
      if (valid[ctx]) return ctx;
      break;
    }
  }
  return is_con ? CONSTRUCTOR : IDENTIFIER;
}

/* ------------------------------------------------------- scan_token */

/* One pass over the token at the cursor (a port of `scan_token` and
   friends). Marks on: the real scan; marks off: the layout lookahead,
   where everything consumed is discarded past the enclosing mark. */
static bool scan_token(Scanner *s, TSLexer *lexer, TokenClass *out) {
  out->kind = KIND_OTHER;
  out->symbol = -1;
  out->word[0] = 0;

  if (s->attr_state == 1) {
    if (!scan_attr_name(s, lexer)) return false;
    out->symbol = ATTR_NAME;
    return true;
  }
  if (s->attr_state == 2) {
    char c = peekc(lexer);
    if (c == ']') {
      take(s, lexer, false);
      mark(s, lexer);
      out->symbol = RBRACKET;
      out->kind = KIND_ATTR_CLOSE;
      return true;
    }
    if (c == '(' || c == '=') {
      if (!scan_attr_args(s, lexer)) return false;
      out->symbol = ATTR_ARGS;
      return true;
    }
    return false;
  }

  char c0 = peekc(lexer);

  if (c0 == ')') {
    take(s, lexer, false);
    mark(s, lexer);
    out->symbol = RPAREN;
    out->kind = KIND_RPAREN;
    return true;
  }
  if (c0 == '}') {
    take(s, lexer, false);
    mark(s, lexer);
    out->symbol = RBRACE;
    out->kind = KIND_RBRACE;
    return true;
  }
  if (c0 == ']') {
    take(s, lexer, false);
    mark(s, lexer);
    out->symbol = RBRACKET;
    out->kind = KIND_RBRACKET;
    return true;
  }
  if (c0 == ',') {
    take(s, lexer, false);
    mark(s, lexer);
    out->symbol = COMMA;
    return true;
  }
  if (c0 == ';') {
    take(s, lexer, false);
    mark(s, lexer);
    out->symbol = SEMICOLON;
    return true;
  }
  if (c0 == '.') {
    take(s, lexer, false);
    mark(s, lexer);
    out->symbol = DOT;
    return true;
  }
  if (c0 == '\\') {
    take(s, lexer, false);
    mark(s, lexer);
    out->symbol = BACKSLASH;
    return true;
  }
  if (c0 == '"') {
    if (!scan_string(s, lexer)) return false;
    out->symbol = STRING_LITERAL;
    return true;
  }
  if (c0 == '(') {
    if (s->bang_pending) {
      take(s, lexer, false);
      if (!scan_token_tree(s, lexer, 1, ')')) return false;
      out->symbol = TOKEN_TREE;
      return true;
    }
    take(s, lexer, false);
    mark(s, lexer);
    out->symbol = LPAREN;
    out->kind = KIND_LPAREN;
    return true;
  }
  if (c0 == '[') {
    take(s, lexer, false);
    if (peekc(lexer) == '|') {
      take(s, lexer, false);
      mark(s, lexer);
      out->symbol = LQUOTE;
      out->kind = KIND_LQUOTE;
      return true;
    }
    if (s->bang_pending) {
      if (!scan_token_tree(s, lexer, 1, ']')) return false;
      out->symbol = TOKEN_TREE;
      return true;
    }
    mark(s, lexer);
    out->symbol = LBRACKET;
    out->kind = KIND_LBRACKET;
    return true;
  }
  if (c0 == '{') {
    take(s, lexer, false);
    if (peekc(lexer) == '-') {
      /* Block comment (nested): trivia. */
      take(s, lexer, false);
      unsigned depth = 1;
      while (depth > 0 && !at_eof(lexer)) {
        char e = peekc(lexer);
        if (e == '{') {
          take(s, lexer, false);
          if (peekc(lexer) == '-') {
            take(s, lexer, false);
            depth++;
          }
        } else if (e == '-') {
          take(s, lexer, false);
          if (peekc(lexer) == '}') {
            take(s, lexer, false);
            depth--;
          }
        } else {
          take(s, lexer, false);
        }
      }
      if (depth != 0) return false;
      mark(s, lexer);
      out->symbol = COMMENT;
      out->kind = KIND_COMMENT;
      return true;
    }
    if (s->bang_pending) {
      if (!scan_token_tree(s, lexer, 1, '}')) return false;
      out->symbol = TOKEN_TREE;
      return true;
    }
    if (s->prim_active && s->prim_seen_where &&
        is_family_word(s->last_word)) {
      if (!scan_fragment(s, lexer)) return false;
      out->symbol = PRIM_FRAGMENT;
      return true;
    }
    mark(s, lexer);
    out->symbol = LBRACE;
    out->kind = KIND_LBRACE;
    return true;
  }
  if (c0 == '|') {
    take(s, lexer, false);
    if (peekc(lexer) == ']') {
      take(s, lexer, false);
      mark(s, lexer);
      out->symbol = RQUOTE;
      out->kind = KIND_RQUOTE;
      return true;
    }
    char run[MAX_WORD];
    size_t rlen = 0;
    run[rlen++] = '|';
    mark(s, lexer);
    scan_run(s, lexer, run, &rlen);
    out->symbol = operator_symbol(run);
    if (out->symbol < 0) out->symbol = OPERATOR;
    return true;
  }
  if (c0 == '$') {
    take(s, lexer, false);
    if (peekc(lexer) == '(') {
      take(s, lexer, false);
      mark(s, lexer);
      out->symbol = SPLICE_OPEN;
      out->kind = KIND_SPLICE;
      return true;
    }
    mark(s, lexer);
    out->symbol = DOLLAR;
    return true;
  }
  if (c0 == '#') {
    take(s, lexer, false);
    if (peekc(lexer) == '[') {
      take(s, lexer, false);
      mark(s, lexer);
      out->symbol = ATTR_OPEN;
      return true;
    }
    char run[MAX_WORD];
    size_t rlen = 0;
    run[rlen++] = '#';
    mark(s, lexer);
    scan_run(s, lexer, run, &rlen);
    out->symbol = operator_symbol(run);
    if (out->symbol < 0) out->symbol = OPERATOR;
    return true;
  }
  if (c0 == '\'') {
    /* Tick syntax (D123, amendments section 12): tick, one character,
       tick is a `Char` and wins; `''T` is a type Name; `'foo` a Sym. */
    take(s, lexer, false); /* the tick */
    char c1 = peekc(lexer);
    if (c1 == '\\') {
      take(s, lexer, false);
      if (!scan_escape(s, lexer)) return false;
      mark(s, lexer);
      if (peekc(lexer) != '\'') return false;
      take(s, lexer, false);
      mark(s, lexer);
      out->symbol = CHAR_LITERAL;
      return true;
    }
    if (c1 == '\'') {
      take(s, lexer, false);
      if (!is_word_start(peekc(lexer))) return false;
      size_t len = 0;
      scan_word_rest(s, lexer, out->word, &len);
      out->word[len] = 0;
      out->symbol = TYPE_NAME_LITERAL;
      return true;
    }
    if (c1 == '\n' || c1 == '\r' || at_eof(lexer)) return false;
    take(s, lexer, false); /* the one character */
    if (peekc(lexer) == '\'') {
      take(s, lexer, false);
      mark(s, lexer);
      out->symbol = CHAR_LITERAL;
      return true;
    }
    /* A Sym literal: the word continues from the character taken. */
    size_t len = 0;
    if (len + 1 < MAX_WORD) out->word[len++] = c1;
    mark(s, lexer);
    scan_word_rest(s, lexer, out->word, &len);
    out->word[len] = 0;
    out->symbol = SYMBOL_LITERAL;
    return true;
  }
  if (is_digit(c0)) {
    int n = scan_number(s, lexer);
    if (n < 0) return false;
    out->symbol = n;
    return true;
  }
  if (is_word_start(c0)) {
    size_t len = 0;
    scan_word_rest(s, lexer, out->word, &len);
    out->word[len] = 0;
    out->kind = KIND_WORD;
    out->symbol = -1;
    return true;
  }
  if (c0 == '-') {
    /* D102/D129: comment, negative literal, or symbol run. */
    char prev = s->prev_char;
    take(s, lexer, false);
    if (peekc(lexer) == '-') {
      /* A line comment: comments win over symbol runs. */
      while (peekc(lexer) != '\n' && !at_eof(lexer))
        take(s, lexer, false);
      mark(s, lexer);
      out->symbol = COMMENT;
      out->kind = KIND_COMMENT;
      return true;
    }
    if (sign_minus_context(prev, peekc(lexer))) {
      int n = scan_number(s, lexer);
      if (n < 0) return false;
      out->symbol = n;
      return true;
    }
    char run[MAX_WORD];
    size_t rlen = 0;
    run[rlen++] = '-';
    mark(s, lexer);
    scan_run(s, lexer, run, &rlen);
    out->symbol = operator_symbol(run);
    if (out->symbol < 0) out->symbol = OPERATOR;
    return true;
  }
  if (is_symbol_char(c0)) {
    if (c0 == '%') {
      char run[MAX_WORD];
      size_t rlen = 0;
      scan_percent(s, lexer, run, &rlen);
      if (rlen == 1 && run[0] <= 4 && run[0] >= 1) {
        /* a multiplicity was encoded in the run text */
        out->symbol = run[0] == 1   ? MULT_ZERO
                      : run[0] == 2 ? MULT_ONE
                      : run[0] == 3 ? MULT_AFF
                                    : MULT_MANY;
        return true;
      }
      out->symbol = operator_symbol(run);
      if (out->symbol < 0) out->symbol = OPERATOR;
      return true;
    }
    char run[MAX_WORD];
    size_t rlen = 0;
    run[rlen++] = c0;
    take(s, lexer, false);
    mark(s, lexer);
    scan_run(s, lexer, run, &rlen);
    out->symbol = operator_symbol(run);
    if (out->symbol < 0) out->symbol = OPERATOR;
    return true;
  }
  return false;
}

/* ------------------------------------------------------------ layout */

static bool frame_is_block(uint8_t kind) {
  return kind == FRAME_BLOCK || kind == FRAME_BLOCK_LET;
}

/* layout.rs, statement for statement. Runs for a real token before
   that token is scanned (from the previous call's lookahead), so the
   virtual tokens it queues are emitted first. */
static void layout_machine(Scanner *s, const TokenClass *tc, uint32_t tok_col,
                           uint32_t tok_line) {
  bool check = true;
  bool is_lparen_like = tc->kind == KIND_LPAREN || tc->kind == KIND_SPLICE;
  bool with_group_open = s->with_paren_pending && is_lparen_like;
  s->with_paren_pending = false;
  bool supply_in = s->supply_close_pending;
  s->supply_close_pending = false;

  if (s->pending_layout) {
    s->pending_layout = false;
    if (tc->kind == KIND_EOF) {
      queue_push(s, LAYOUT_OPEN);
      queue_push(s, LAYOUT_CLOSE);
    } else if (tc->kind == KIND_LBRACE) {
      /* An explicit `{` supplies the block verbatim. */
      if (s->n_frames < MAX_FRAMES) {
        Frame *f = &s->frames[s->n_frames++];
        f->kind = FRAME_BRACE;
        f->col = 0;
      }
      s->prev_tok_line = tok_line;
      s->prev_tok_valid = true;
      return;
    } else {
      bool too_shallow = false;
      if (s->n_frames > 0 &&
          frame_is_block(s->frames[s->n_frames - 1].kind) &&
          tok_col <= s->frames[s->n_frames - 1].col) {
        /* layout.rs rejects this ("must indent farther"); we refuse to
           open the block so the parse reports the divergence. */
        too_shallow = true;
      }
      if (!too_shallow) {
        if (s->n_frames < MAX_FRAMES) {
          Frame *f = &s->frames[s->n_frames++];
          f->kind = s->pending_layout_is_let ? FRAME_BLOCK_LET : FRAME_BLOCK;
          f->col = (uint16_t)tok_col;
        }
        queue_push(s, LAYOUT_OPEN);
        check = false;
      }
    }
  }

  if (tc->kind == KIND_EOF) {
    while (s->n_frames > 0 && frame_is_block(s->frames[s->n_frames - 1].kind)) {
      s->n_frames--;
      queue_push(s, LAYOUT_CLOSE);
    }
    return;
  }

  if (tc->kind == KIND_RPAREN || tc->kind == KIND_RBRACKET ||
      tc->kind == KIND_RBRACE) {
    while (s->n_frames > 0 && frame_is_block(s->frames[s->n_frames - 1].kind)) {
      s->n_frames--;
      queue_push(s, LAYOUT_CLOSE);
    }
    uint8_t want = tc->kind == KIND_RPAREN
                       ? FRAME_PAREN
                       : (tc->kind == KIND_RBRACKET ? FRAME_BRACKET
                                                    : FRAME_BRACE);
    if (s->n_frames > 0 && s->frames[s->n_frames - 1].kind == want) {
      bool was_with = s->frames[s->n_frames - 1].col == 1;
      s->n_frames--;
      if (was_with) s->supply_close_pending = true;
    }
    /* mismatched delimiters are the parser's to reject */
    s->prev_tok_line = tok_line;
    s->prev_tok_valid = true;
    return;
  }

  if (tc->kind == KIND_WORD && strcmp(tc->word, "in") == 0) {
    if (supply_in) {
      /* The supply clause's own `in`: closes nothing. */
      s->prev_tok_line = tok_line;
      s->prev_tok_valid = true;
      return;
    }
    /* Close the nearest implicit `let` block plus nested blocks, not
       crossing an explicit delimiter. */
    long let_at = -1;
    for (long i = (long)s->n_frames - 1; i >= 0; i--) {
      if (!frame_is_block(s->frames[i].kind)) break;
      if (s->frames[i].kind == FRAME_BLOCK_LET) {
        let_at = i;
        break;
      }
    }
    if (let_at >= 0) {
      while ((long)s->n_frames > let_at) {
        s->n_frames--;
        queue_push(s, LAYOUT_CLOSE);
      }
    }
    s->prev_tok_line = tok_line;
    s->prev_tok_valid = true;
    return;
  }

  if (tc->kind == KIND_WORD && strcmp(tc->word, "with") == 0) {
    s->with_paren_pending = true;
  }

  if (check && s->prev_tok_valid && tok_line != s->prev_tok_line) {
    while (s->n_frames > 0 && frame_is_block(s->frames[s->n_frames - 1].kind) &&
           s->frames[s->n_frames - 1].col > tok_col) {
      s->n_frames--;
      queue_push(s, LAYOUT_CLOSE);
    }
    if (s->n_frames > 0 && frame_is_block(s->frames[s->n_frames - 1].kind) &&
        s->frames[s->n_frames - 1].col == tok_col) {
      queue_push(s, LAYOUT_SEMI);
      /* A `name!(...)` split here must not glue. */
      s->bang_pending = false;
    }
  }

  if (tc->kind == KIND_WORD) {
    if (strcmp(tc->word, "where") == 0 || strcmp(tc->word, "of") == 0 ||
        strcmp(tc->word, "do") == 0 || strcmp(tc->word, "let") == 0 ||
        strcmp(tc->word, "letrec") == 0) {
      s->pending_layout = true;
      s->pending_layout_is_let = strcmp(tc->word, "let") == 0 ||
                                strcmp(tc->word, "letrec") == 0;
    }
  } else if (is_lparen_like || tc->kind == KIND_LBRACKET ||
             tc->kind == KIND_LBRACE) {
    if (s->n_frames < MAX_FRAMES) {
      Frame *f = &s->frames[s->n_frames++];
      f->kind = tc->kind == KIND_LBRACKET
                    ? FRAME_BRACKET
                    : (tc->kind == KIND_LBRACE ? FRAME_BRACE : FRAME_PAREN);
      f->col = with_group_open ? 1 : 0;
    }
  }

  s->prev_tok_line = tok_line;
  s->prev_tok_valid = true;
}

/* lexer.rs `note`: the prim context, `last_word`, and the attribute /
   macro-call flags. Runs for real tokens only. */
static void note_token(Scanner *s, const TokenClass *tc, bool tok_line_start,
                       uint32_t tok_col) {
  if (tc->kind == KIND_WORD) {
    if (is_reserved_word(tc->word)) {
      s->last_word[0] = 0;
      if (strcmp(tc->word, "prim") == 0) {
        s->prim_active = true;
        s->prim_col = (uint16_t)tok_col;
        s->prim_seen_where = false;
        return;
      }
      if (s->prim_active && strcmp(tc->word, "where") == 0) {
        s->prim_seen_where = true;
      }
    } else {
      strncpy(s->last_word, tc->word, MAX_WORD - 1);
      s->last_word[MAX_WORD - 1] = 0;
    }
  } else {
    s->last_word[0] = 0;
  }
  if (s->prim_active && tok_line_start && tok_col <= s->prim_col) {
    s->prim_active = false;
  }
  if (tc->symbol == ATTR_OPEN) {
    s->attr_state = 1;
  } else if (tc->symbol == ATTR_NAME) {
    s->attr_state = 2;
  } else if (tc->symbol == RBRACKET && s->attr_state == 2) {
    s->attr_state = 0;
  } else if (tc->symbol != ATTR_ARGS) {
    s->attr_state = 0;
  }
  if (tc->symbol == BANG) {
    s->bang_pending = true;
  } else {
    s->bang_pending = false;
  }
}

/* Classify the next real token mark-free (comments are trivia; end of
   input is KIND_EOF). All input consumed here is discarded past the
   enclosing token's mark. */
static void classify_next(Scanner *s, TSLexer *lexer, TokenClass *out,
                          uint32_t *tok_col, uint32_t *tok_line) {
  /* Everything consumed past the final mark is replayed: start from
     the mark and seed the tracking from its record. */
  r_rewind();
  Scanner tmp = *s;
  tmp.cur_line = marks.line;
  tmp.cur_col = marks.col;
  tmp.prev_char = marks.prev_char;
  tmp.line_leading = marks.leading;
  for (;;) {
    while (peekc(lexer) == ' ' || peekc(lexer) == '\t' ||
           peekc(lexer) == '\n' || peekc(lexer) == '\r') {
      take(&tmp, lexer, false);
    }
    /* The tracked column is the truth here: during replay the lexer
       cursor sits past the mark and `get_column` would report the
       overflow position. */
    *tok_col = tmp.cur_col;
    *tok_line = tmp.cur_line;
    if (at_eof(lexer)) {
      out->kind = KIND_EOF;
      out->symbol = -1;
      out->word[0] = 0;
      return;
    }
    if (!scan_token(&tmp, lexer, out)) {
      out->kind = KIND_OTHER;
      out->symbol = -1;
      out->word[0] = 0;
      return;
    }
    if (out->kind == KIND_COMMENT) continue;
    return;
  }
}

/* ----------------------------------------------------------------- scan */

bool tree_sitter_safelang_external_scanner_scan(void *payload, TSLexer *lexer,
                                               const bool *valid_symbols) {
  Scanner *s = (Scanner *)payload;
  Scanner snapshot = *s;

  r_len = 0;
  r_pos = 0;
  /* Queued virtual tokens come first (zero width at this position). */
  if (s->q_len > s->q_head) {
    uint8_t sym = s->queue[s->q_head++];
    if (s->q_head == s->q_len) {
      s->q_head = 0;
      s->q_len = 0;
    }
    s->bang_pending = false;
    marks.on = true;
    mark(s, lexer);
    lexer->result_symbol = sym;
    return true;
  }

  /* Whitespace (comments scan as trivia tokens below). */
  while (peekc(lexer) == ' ' || peekc(lexer) == '\t' || peekc(lexer) == '\n' ||
         peekc(lexer) == '\r') {
    take(s, lexer, true);
  }

  if (at_eof(lexer)) {
    if (!s->machine_done) {
      TokenClass eof_tc;
      memset(&eof_tc, 0, sizeof(eof_tc));
      eof_tc.kind = KIND_EOF;
      marks.on = false;
      layout_machine(s, &eof_tc, s->cur_col, s->cur_line);
      marks.on = true;
      s->machine_done = true;
    }
    if (s->q_len > s->q_head) {
      uint8_t sym = s->queue[s->q_head++];
      if (s->q_head == s->q_len) {
        s->q_head = 0;
        s->q_len = 0;
      }
      mark(s, lexer);
      lexer->result_symbol = sym;
      return true;
    }
    return false;
  }

  uint32_t tok_col = s->cur_col;
  uint32_t tok_line = s->cur_line;
  bool tok_line_start = s->line_leading;

  marks.on = true;
  TokenClass tc;
  if (!scan_token(s, lexer, &tc)) {
    *s = snapshot;
    return false;
  }

  if (tc.kind == KIND_COMMENT) {
    /* Comments are trivia: no layout machine, no note. The machine for
       the next real token runs in the lookahead of the call that
       returns it (or of the last real token). */
    s->prev_char = marks.prev_char;
    s->cur_line = marks.line;
    s->cur_col = marks.col;
    s->line_leading = marks.leading;
    lexer->result_symbol = COMMENT;
    return true;
  }

  if (!s->machine_done) {
    /* Input start: the machine for this token could not run in an
       earlier lookahead. It queues no virtual tokens here (no pending
       block, no previous token). */
    layout_machine(s, &tc, tok_col, tok_line);
  }
  note_token(s, &tc, tok_line_start, tok_col);
  s->machine_done = false;

  /* Classify the next real token mark-free (the overflow past this
     token's mark is replayed first). Its class feeds both the
     contextual word decisions and its layout machine, whose virtual
     tokens are then queued before it is scanned. */
  TokenClass next_tc;
  uint32_t next_col = 0, next_line = 0;
  marks.on = false;
  classify_next(s, lexer, &next_tc, &next_col, &next_line);

  int symbol = tc.symbol;
  if (tc.kind == KIND_WORD) {
    symbol = word_symbol(valid_symbols, tc.word,
                         next_tc.kind == KIND_EOF ? NULL : &next_tc);
  }

  layout_machine(s, &next_tc, next_col, next_line);
  s->machine_done = true;
  marks.on = true;

  /* Commit position tracking up to the final mark. */
  s->prev_char = marks.prev_char;
  s->cur_line = marks.line;
  s->cur_col = marks.col;
  s->line_leading = marks.leading;

  lexer->result_symbol = symbol;
  return true;
}
