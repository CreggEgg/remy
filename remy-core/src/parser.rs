use std::default;

use logos::{Lexer, Logos, Skip};
#[derive(Debug, PartialEq, Default)]
pub enum StringState {
    #[default]
    NotStarted,
    Started,
}

#[derive(Debug, PartialEq, Default)]
pub struct LogosState {
    string_state: StringState,
}

#[derive(Logos, Debug, PartialEq)]
#[logos(extras = LogosState)]
pub enum Token<'a> {
    #[regex("//[^\n]*")]
    LineComment,
    #[regex("[ \t\r\n\n]+")]
    WhiteSpace,
    #[regex(r#"/\*[^(*/)]*\*/"#)]
    BlockComment,
    #[token(":")]
    Colon,
    #[token(";")]
    SemiColon,
    #[regex("[_a-zA-Z][_a-zA-Z0-9]*")]
    Ident(&'a str),
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,
    #[token("!")]
    Exclamation,
    #[token("=>")]
    FatArrow,
    #[regex(r#""(?:[^"(\$\{)]|\\")*""#)]
    NormalString(&'a str),
    // #[regex(r#""(?:[^(\$\{)]|\\")*(\$\{)"#)]
    // StartInterpolatedString(&'a str),
    // #[regex(r#"\}(?:[^"(\$\{)]|\\")*""#, interpolated_string_callback)]
    // EndInterpolatedString(&'a str),
    #[token("{")]
    LBrace,
    #[token("}", priority = 1)]
    RBrace,
    #[regex(r#"[0-9]+(\.(0-9)+)?"#)]
    Number(&'a str),
}
