//! Layout pass: converts token streams with significant indentation into the
//! explicit `{`, `;`, `}` form the parser consumes.
//!
//! Rules (draft 0.2): `where`, `of`, `do` and `let` introduce a layout block
//! unless followed by an explicit `{`. The next token starts the block at its
//! column. At a logical line start, equal indentation inserts `;`, lower
//! indentation closes nested implicit blocks, greater indentation continues
//! the current item. The first item of a block receives no separator. An
//! implicit block must indent farther than its enclosing implicit block.
//! Explicit `(`, `[` and `{` shield enclosing layout while open, and a closing
//! `)`, `]` or `}` first closes implicit blocks opened inside that delimiter.
//! `in` closes the nearest implicit `let` block plus anything nested inside it
//! and must not cross an unmatched explicit delimiter. End of file closes all
//! implicit blocks. Inserted `{`, `;` and `}` tokens carry `virtual_: true`.

use crate::lexer::{Span, Tok, Token};
use crate::{Error, Result};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Delim {
    Paren,
    Bracket,
    Brace,
}

enum Frame {
    /// An implicit block opened by `where`, `of`, `do` or `let`; `is_let`
    /// marks blocks that `in` may close.
    Implicit { col: u32, is_let: bool },
    /// An explicit `(`, `[` or `{` shielding enclosing layout.
    Delim(Delim),
}

fn virtual_tok(tok: Tok, span: Span) -> Token {
    Token { tok, span, virtual_: true }
}

/// Insert virtual `{`, `;`, `}` tokens.
pub fn layout(tokens: Vec<Token>) -> Result<Vec<Token>> {
    let mut out: Vec<Token> = Vec::with_capacity(tokens.len());
    let mut stack: Vec<Frame> = Vec::new();
    // A layout context requested by `where`, `of`, `do` or `let`; the block
    // opens at the next token. The flag records a `let` context.
    let mut pending: Option<(Span, bool)> = None;
    let mut prev: Option<Span> = None;

    for tok in tokens {
        let span = tok.span;
        // The token that opens a block is its first item: no separator checks.
        let mut check = true;

        if let Some((_, is_let)) = pending.take() {
            if tok.tok == Tok::Eof {
                out.push(virtual_tok(Tok::LBrace, span));
                out.push(virtual_tok(Tok::RBrace, span));
            } else if tok.tok == Tok::LBrace {
                // An explicit `{` supplies the block verbatim.
                out.push(tok);
                stack.push(Frame::Delim(Delim::Brace));
                prev = Some(span);
                continue;
            } else {
                if let Some(Frame::Implicit { col, .. }) = stack.last() {
                    if span.col <= *col {
                        return Err(Error::at(
                            span,
                            format!(
                                "layout block must indent farther than its enclosing block at column {col}, but this one starts at column {}",
                                span.col
                            ),
                        ));
                    }
                }
                stack.push(Frame::Implicit { col: span.col, is_let });
                out.push(virtual_tok(Tok::LBrace, span));
                check = false;
            }
        }

        match tok.tok {
            Tok::Eof => {
                while let Some(Frame::Implicit { .. }) = stack.last() {
                    stack.pop();
                    out.push(virtual_tok(Tok::RBrace, span));
                }
                out.push(tok);
                return Ok(out);
            }
            Tok::RParen | Tok::RBracket | Tok::RBrace => {
                while matches!(stack.last(), Some(Frame::Implicit { .. })) {
                    stack.pop();
                    out.push(virtual_tok(Tok::RBrace, span));
                }
                let want = match tok.tok {
                    Tok::RParen => Delim::Paren,
                    Tok::RBracket => Delim::Bracket,
                    _ => Delim::Brace,
                };
                if !matches!(stack.pop(), Some(Frame::Delim(d)) if d == want) {
                    return Err(Error::at(span, "mismatched closing delimiter"));
                }
                out.push(tok);
                prev = Some(span);
                continue;
            }
            Tok::In => {
                // Close the nearest implicit `let` block plus nested implicit
                // blocks, without crossing an explicit delimiter.
                let mut let_at = None;
                let mut crossed_delim = false;
                for (i, f) in stack.iter().enumerate().rev() {
                    match f {
                        Frame::Delim(_) => {
                            crossed_delim = true;
                            break;
                        }
                        Frame::Implicit { is_let: true, .. } => {
                            let_at = Some(i);
                            break;
                        }
                        Frame::Implicit { .. } => {}
                    }
                }
                let Some(let_at) = let_at else {
                    return Err(Error::at(
                        span,
                        if crossed_delim {
                            "`in` must not cross an unmatched explicit delimiter"
                        } else {
                            "`in` has no matching implicit `let` block"
                        },
                    ));
                };
                while stack.len() > let_at {
                    stack.pop();
                    out.push(virtual_tok(Tok::RBrace, span));
                }
                out.push(tok);
                prev = Some(span);
                continue;
            }
            _ => {}
        }

        if check {
            if let Some(p) = prev {
                if span.line != p.line {
                    // Logical line start: close implicit blocks the token has
                    // outdented past, then separate equal indentation.
                    while let Some(Frame::Implicit { col, .. }) = stack.last() {
                        if span.col < *col {
                            stack.pop();
                            out.push(virtual_tok(Tok::RBrace, span));
                        } else {
                            break;
                        }
                    }
                    if let Some(Frame::Implicit { col, .. }) = stack.last() {
                        if span.col == *col {
                            out.push(virtual_tok(Tok::Semicolon, span));
                        }
                    }
                }
            }
        }

        let opens_layout = matches!(tok.tok, Tok::Where | Tok::Of | Tok::Do | Tok::Let);
        let is_open_delim =
            matches!(tok.tok, Tok::LParen | Tok::LBracket | Tok::LBrace);
        out.push(tok);
        if opens_layout {
            pending = Some((span, false));
            if let Some(p) = out.last_mut() {
                if p.tok == Tok::Let {
                    pending = Some((span, true));
                }
            }
        } else if is_open_delim {
            let d = match out.last().map(|t| &t.tok) {
                Some(Tok::LParen) => Delim::Paren,
                Some(Tok::LBracket) => Delim::Bracket,
                _ => Delim::Brace,
            };
            stack.push(Frame::Delim(d));
        }
        prev = Some(span);
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn kinds(src: &str) -> Vec<Tok> {
        let toks = crate::lexer::lex(src).expect("lex");
        layout(toks).expect("layout").into_iter().map(|t| t.tok).collect()
    }

    fn layout_err(src: &str) -> Option<crate::Error> {
        match crate::lexer::lex(src) {
            Err(e) => Some(e),
            Ok(toks) => layout(toks).err(),
        }
    }

    #[test]
    fn equal_indent_separates_lower_indent_closes() {
        assert_eq!(
            kinds("module M where\n  a = 1\n  b = 2\n"),
            vec![
                Tok::Module,
                Tok::ConId("M".into()),
                Tok::Where,
                Tok::LBrace,
                Tok::VarId("a".into()),
                Tok::Equals,
                Tok::Int("1".to_string()),
                Tok::Semicolon,
                Tok::VarId("b".into()),
                Tok::Equals,
                Tok::Int("2".to_string()),
                Tok::RBrace,
                Tok::Eof
            ]
        );
    }

    #[test]
    fn greater_indent_continues_the_item() {
        assert_eq!(
            kinds("module M where\n  a = 1\n    + 2\n  b = 3\n"),
            vec![
                Tok::Module,
                Tok::ConId("M".into()),
                Tok::Where,
                Tok::LBrace,
                Tok::VarId("a".into()),
                Tok::Equals,
                Tok::Int("1".to_string()),
                Tok::Op("+".into()),
                Tok::Int("2".to_string()),
                Tok::Semicolon,
                Tok::VarId("b".into()),
                Tok::Equals,
                Tok::Int("3".to_string()),
                Tok::RBrace,
                Tok::Eof
            ]
        );
    }

    #[test]
    fn nested_blocks_close_on_outdent() {
        assert_eq!(
            kinds("case x of\n  A -> case y of\n    B -> 1\n    C -> 2\n  D -> 3\n"),
            vec![
                Tok::Case,
                Tok::VarId("x".into()),
                Tok::Of,
                Tok::LBrace,
                Tok::ConId("A".into()),
                Tok::Arrow,
                Tok::Case,
                Tok::VarId("y".into()),
                Tok::Of,
                Tok::LBrace,
                Tok::ConId("B".into()),
                Tok::Arrow,
                Tok::Int("1".to_string()),
                Tok::Semicolon,
                Tok::ConId("C".into()),
                Tok::Arrow,
                Tok::Int("2".to_string()),
                Tok::RBrace,
                Tok::Semicolon,
                Tok::ConId("D".into()),
                Tok::Arrow,
                Tok::Int("3".to_string()),
                Tok::RBrace,
                Tok::Eof
            ]
        );
    }

    #[test]
    fn in_closes_let_block_on_one_line() {
        assert_eq!(
            kinds("let x = 1 in x"),
            vec![
                Tok::Let,
                Tok::LBrace,
                Tok::VarId("x".into()),
                Tok::Equals,
                Tok::Int("1".to_string()),
                Tok::RBrace,
                Tok::In,
                Tok::VarId("x".into()),
                Tok::Eof
            ]
        );
    }

    #[test]
    fn in_closes_let_block_across_lines() {
        assert_eq!(
            kinds("let x = 1\n    y = 2\n  in x"),
            vec![
                Tok::Let,
                Tok::LBrace,
                Tok::VarId("x".into()),
                Tok::Equals,
                Tok::Int("1".to_string()),
                Tok::Semicolon,
                Tok::VarId("y".into()),
                Tok::Equals,
                Tok::Int("2".to_string()),
                Tok::RBrace,
                Tok::In,
                Tok::VarId("x".into()),
                Tok::Eof
            ]
        );
    }

    #[test]
    fn in_closes_nested_blocks_above_let() {
        assert_eq!(
            kinds("let x = case y of\n      A -> 1\n  in x"),
            vec![
                Tok::Let,
                Tok::LBrace,
                Tok::VarId("x".into()),
                Tok::Equals,
                Tok::Case,
                Tok::VarId("y".into()),
                Tok::Of,
                Tok::LBrace,
                Tok::ConId("A".into()),
                Tok::Arrow,
                Tok::Int("1".to_string()),
                Tok::RBrace,
                Tok::RBrace,
                Tok::In,
                Tok::VarId("x".into()),
                Tok::Eof
            ]
        );
    }

    #[test]
    fn explicit_braces_supply_the_block_verbatim() {
        assert_eq!(
            kinds("module M where {\n  a = 1; b = 2 }"),
            vec![
                Tok::Module,
                Tok::ConId("M".into()),
                Tok::Where,
                Tok::LBrace,
                Tok::VarId("a".into()),
                Tok::Equals,
                Tok::Int("1".to_string()),
                Tok::Semicolon,
                Tok::VarId("b".into()),
                Tok::Equals,
                Tok::Int("2".to_string()),
                Tok::RBrace,
                Tok::Eof
            ]
        );
    }

    #[test]
    fn explicit_delimiters_shield_enclosing_layout() {
        // `h` is outdented relative to the module block but sits inside
        // parentheses, so no implicit block closes and no separator appears.
        assert_eq!(
            kinds("module M where\n  f = (g\n  h)\n  i = 2\n"),
            vec![
                Tok::Module,
                Tok::ConId("M".into()),
                Tok::Where,
                Tok::LBrace,
                Tok::VarId("f".into()),
                Tok::Equals,
                Tok::LParen,
                Tok::VarId("g".into()),
                Tok::VarId("h".into()),
                Tok::RParen,
                Tok::Semicolon,
                Tok::VarId("i".into()),
                Tok::Equals,
                Tok::Int("2".to_string()),
                Tok::RBrace,
                Tok::Eof
            ]
        );
    }

    #[test]
    fn closing_delimiter_closes_inner_implicit_blocks() {
        assert_eq!(
            kinds("f (case x of\n  A -> 1)"),
            vec![
                Tok::VarId("f".into()),
                Tok::LParen,
                Tok::Case,
                Tok::VarId("x".into()),
                Tok::Of,
                Tok::LBrace,
                Tok::ConId("A".into()),
                Tok::Arrow,
                Tok::Int("1".to_string()),
                Tok::RBrace,
                Tok::RParen,
                Tok::Eof
            ]
        );
    }

    #[test]
    fn do_block_with_let_statement_closes_by_indentation() {
        assert_eq!(
            kinds("do\n  let x = 1\n  f x"),
            vec![
                Tok::Do,
                Tok::LBrace,
                Tok::Let,
                Tok::LBrace,
                Tok::VarId("x".into()),
                Tok::Equals,
                Tok::Int("1".to_string()),
                Tok::RBrace,
                Tok::Semicolon,
                Tok::VarId("f".into()),
                Tok::VarId("x".into()),
                Tok::RBrace,
                Tok::Eof
            ]
        );
    }

    #[test]
    fn eof_closes_all_implicit_blocks() {
        assert_eq!(
            kinds("module M where\n  f = let x = 1"),
            vec![
                Tok::Module,
                Tok::ConId("M".into()),
                Tok::Where,
                Tok::LBrace,
                Tok::VarId("f".into()),
                Tok::Equals,
                Tok::Let,
                Tok::LBrace,
                Tok::VarId("x".into()),
                Tok::Equals,
                Tok::Int("1".to_string()),
                Tok::RBrace,
                Tok::RBrace,
                Tok::Eof
            ]
        );
    }

    #[test]
    fn empty_block_after_where_at_eof() {
        assert_eq!(
            kinds("module M where"),
            vec![
                Tok::Module,
                Tok::ConId("M".into()),
                Tok::Where,
                Tok::LBrace,
                Tok::RBrace,
                Tok::Eof
            ]
        );
    }

    #[test]
    fn in_must_not_cross_an_explicit_delimiter() {
        let e = layout_err("let x = (1 in 2)").expect("error");
        assert!(e.message.contains("must not cross"), "{e}");
    }

    #[test]
    fn in_without_let_block_is_an_error() {
        let e = layout_err("f = 1 in 2").expect("error");
        assert!(e.message.contains("no matching implicit `let`"), "{e}");
    }

    #[test]
    fn implicit_block_must_indent_farther() {
        let e = layout_err("module M where\n  f = g x\n   where\n  h = 1").expect("error");
        assert!(e.message.contains("must indent farther"), "{e}");
    }

    #[test]
    fn mismatched_delimiter_is_an_error() {
        assert!(layout_err("f = (a]").is_some());
    }

    #[test]
    fn tabs_rejected_by_the_lexer_before_layout() {
        // The layout pass sees only post-lex streams; tab rejection is the
        // lexer's and must surface unchanged.
        let e = layout_err("module M where\n\tfoo = 1").expect("error");
        assert!(e.message.contains("tabs"), "{e}");
    }

    #[test]
    fn inserted_tokens_are_marked_virtual() {
        let toks = crate::lexer::lex("module M where\n  a = 1\n  b = 2\n").unwrap();
        let out = layout(toks).unwrap();
        let virt: Vec<Tok> = out
            .iter()
            .filter(|t| t.virtual_)
            .map(|t| t.tok.clone())
            .collect();
        assert_eq!(virt, vec![Tok::LBrace, Tok::Semicolon, Tok::RBrace]);
    }
}
