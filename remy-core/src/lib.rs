use std::ops::Range;

use ast::File;
use chumsky::{error::Simple, Parser, Span};
use logos::Logos;
use parser::{Spanned, Token};

pub mod ast;
pub mod parser;

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

#[cfg(test)]
mod tests {
    use ast::{AnnotatedIdent, Expr, File, Literal, TopLevelDefinition, TypeName};

    use logos::Logos;
    #[cfg(test)]
    use pretty_assertions::{assert_eq, assert_ne};

    use super::*;

    const FULL_EXAMPLE: &str = r#"
// hi
/*
 this too
*/
main :: (args: [string]) => {
    println("Hello to:");
    // println("${args[0]!!}");
}
"#;

    #[test]
    fn lexing() {
        let mut toks = vec![];
        let mut res = parser::Token::lexer(FULL_EXAMPLE);
        while let Some(tok) = res.next() {
            if let Ok(tok) = tok {
                toks.push(tok);
            } else {
                let span = &FULL_EXAMPLE[res.span()];
                dbg!(toks);
                panic!("Failed to parse@{:?}: {}", res.span(), span)
            }
            // res.span();
        }

        use crate::parser::Token;
        assert_eq!(
            toks,
            vec![
                // Token::WhiteSpace,
                // Token::LineComment,
                // Token::WhiteSpace,
                // Token::BlockComment,
                // Token::WhiteSpace,
                Token::Ident("main"),
                // Token::WhiteSpace,
                Token::DoubleColon,
                // Token::WhiteSpace,
                Token::LParen,
                Token::Ident("args"),
                Token::Colon,
                // Token::WhiteSpace,
                Token::LBracket,
                Token::Ident("string"),
                Token::RBracket,
                Token::RParen,
                // Token::WhiteSpace,
                Token::FatArrow,
                // Token::WhiteSpace,
                Token::LBrace,
                // Token::WhiteSpace,
                Token::Ident("println"),
                Token::LParen,
                Token::NormalString("\"Hello to:\""),
                Token::RParen,
                Token::SemiColon,
                // Token::WhiteSpace,
                // Token::LineComment,
                // Token::WhiteSpace,
                // Token::Ident("println"),
                // Token::LParen,
                // Token::StartInterpolatedString("\"${"),
                // Token::Ident("args"),
                // Token::LBracket,
                // Token::Number("0"),
                // Token::RBracket,
                // Token::Exclamation,
                // Token::Exclamation,
                // Token::EndInterpolatedString("}\""),
                // Token::RParen,
                // Token::SemiColon,
                // Token::WhiteSpace,
                Token::RBrace,
                // Token::WhiteSpace,
            ]
        );
    }
    #[test]
    fn parsing() {
        assert_eq!(
            parse(
                r#"
    // hi
    /*
     this too
    */
    main :: (args: [string]) {
        println("Hello world")
    }
        "#
            )
            .unwrap(),
            File {
                definitions: vec![TopLevelDefinition::Binding {
                    name: "main".into(),
                    rhs: ast::Literal::Function {
                        args: vec![AnnotatedIdent {
                            name: "args".into(),
                            r#type: ast::TypeName::Slice(Box::new(TypeName::Named(
                                "string".into()
                            )))
                        }],
                        body: vec![Expr::FunctionCall(
                            Box::new(Expr::Ident("println".into())),
                            vec![Expr::Literal(Literal::String("Hello world".into()))]
                        )]
                    }
                }]
            }
        )
    }
}
