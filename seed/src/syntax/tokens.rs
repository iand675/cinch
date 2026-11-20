//! Structural readers for captured token sequences. Slices keep the original
//! token spans and receive the original scope when wrapped by the stage API.
use crate::lexer::{Tok, Token};
use crate::{Error, Result};

fn opening(token: &Tok) -> Option<u8> {
    match token { Tok::LParen | Tok::DollarParen => Some(0), Tok::LBracket => Some(1),
        Tok::LBrace => Some(2), Tok::LQuote => Some(3), _ => None }
}
fn closing(token: &Tok) -> Option<u8> {
    match token { Tok::RParen => Some(0), Tok::RBracket => Some(1),
        Tok::RBrace => Some(2), Tok::RQuote => Some(3), _ => None }
}

fn boundaries(tokens: &[Token]) -> Result<Vec<usize>> {
    let mut stack = Vec::new();
    let mut commas = Vec::new();
    for (index, token) in tokens.iter().enumerate() {
        if let Some(kind) = opening(&token.tok) { stack.push(kind); }
        else if let Some(kind) = closing(&token.tok) {
            if stack.pop() != Some(kind) { return Err(Error::at(token.span, "mismatched captured syntax delimiter")); }
        } else if matches!(token.tok, Tok::Comma) && stack.is_empty() { commas.push(index); }
    }
    if !stack.is_empty() { return Err(Error::plain("unfinished captured syntax delimiter")); }
    Ok(commas)
}

/// Ignore parentheses enclosing one item, while retaining tuple delimiters.
pub(crate) fn unparenthesized(mut tokens: &[Token]) -> Result<&[Token]> {
    while tokens.len() > 2 && matches!(tokens.first().map(|t| &t.tok), Some(Tok::LParen))
        && matches!(tokens.last().map(|t| &t.tok), Some(Tok::RParen))
    {
        // This also proves the outer pair encloses the whole token sequence:
        // a close/open between them would mismatch the inner stack.
        let inner = &tokens[1..tokens.len()-1];
        if !boundaries(inner)?.is_empty() { break; }
        tokens = inner;
    }
    Ok(tokens)
}

pub(crate) fn items(tokens: &[Token]) -> Result<Vec<Vec<Token>>> {
    let tokens = unparenthesized(tokens)?;
    let pair = (tokens.first().map(|t| &t.tok), tokens.last().map(|t| &t.tok));
    if !matches!(pair, (Some(Tok::LBracket), Some(Tok::RBracket)) | (Some(Tok::LParen), Some(Tok::RParen))) {
        return Err(Error::plain("Syntax.items takes a captured list or tuple"));
    }
    let inner = &tokens[1..tokens.len()-1];
    if inner.is_empty() { return Ok(Vec::new()); }
    let mut cuts = boundaries(inner)?;
    cuts.push(inner.len());
    let mut start = 0;
    let mut result = Vec::new();
    for end in cuts {
        if start == end { return Err(Error::plain("empty item in captured syntax sequence")); }
        result.push(inner[start..end].to_vec());
        start = end + 1;
    }
    Ok(result)
}
