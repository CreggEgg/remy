use std::ops::Range;

use ast::File;
use chumsky::{error::Simple, Parser, Span};
use logos::Logos;
use parser::{Spanned, Token};

pub mod ast;
pub mod parser;
#[cfg(test)]
mod tests;

#[derive(Debug)]
pub enum ParseError<'a> {
    UnrecognizedToken(Range<usize>),
    ParseFailed(Vec<Simple<Token<'a>>>),
}

pub fn parse(file: &str) -> Result<File, ParseError<'_>> {
    let mut tokens = vec![];
    let mut lexer = parser::Token::lexer(file);
    while let Some(tok) = lexer.next() {
        tokens.push(match tok {
            Ok(tok) => Ok(/* Spanned( */ tok /* , lexer.span()) */),
            Err(_) => Err(ParseError::UnrecognizedToken(lexer.span())),
        }?);
    }
    dbg!(&tokens);
    parser::file()
        .parse(tokens)
        .map_err(|err| ParseError::ParseFailed(err)) //.replace_err()
}
