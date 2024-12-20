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
    5+5* 6 / 90 - 20;
    // println("${args[0]!!}");
    x :: true;
    match x 
    | true => println("this will always happen")
    | false => println("this will never happen")
    ;
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
                Token::Number("5"),
                Token::Plus,
                Token::Number("5"),
                Token::Multiply,
                Token::Number("6"),
                Token::Divide,
                Token::Number("90"),
                Token::Minus,
                Token::Number("20"),
                Token::SemiColon,
                Token::Ident("x"),
                Token::DoubleColon,
                Token::Ident("true"),
                Token::SemiColon,
                Token::Match,
                Token::Ident("x"),
                Token::Bar,
                Token::Ident("true"),
                Token::FatArrow,
                Token::Ident("println"),
                Token::LParen,
                Token::NormalString("\"this will always happen\""),
                Token::RParen,
                Token::Bar,
                Token::Ident("false"),
                Token::FatArrow,
                Token::Ident("println"),
                Token::LParen,
                Token::NormalString("\"this will never happen\""),
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
    fn parsing_simple() {
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
    #[test]
    fn parsing_binary_operations() {
        assert_eq!(
            parse(
                r#"
    // hi
    /*
     this too
    */
    main :: (args: [string]) {
        println(5+5*6);
        println(5+5+4+4-4*6*20/15)
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
                        body: vec![
                            Expr::FunctionCall(
                                Box::new(Expr::Ident("println".into())),
                                vec![Expr::BinaryOp {
                                    op: ast::BinaryOperator::Add,
                                    lhs: Box::new(Expr::Literal(Literal::Int(5))),
                                    rhs: Box::new(Expr::BinaryOp {
                                        op: ast::BinaryOperator::Multiply,
                                        lhs: Box::new(Expr::Literal(Literal::Int(5))),
                                        rhs: Box::new(Expr::Literal(Literal::Int(6)))
                                    })
                                }]
                            ),
                            Expr::FunctionCall(
                                Box::new(Expr::Ident("println".into())),
                                vec![Expr::BinaryOp {
                                    op: ast::BinaryOperator::Subtract,
                                    lhs: Box::new(Expr::BinaryOp {
                                        op: ast::BinaryOperator::Add,
                                        lhs: Box::new(Expr::BinaryOp {
                                            op: ast::BinaryOperator::Add,
                                            lhs: Box::new(Expr::BinaryOp {
                                                op: ast::BinaryOperator::Add,
                                                lhs: Box::new(Expr::Literal(Literal::Int(5,),)),
                                                rhs: Box::new(Expr::Literal(Literal::Int(5,),)),
                                            }),
                                            rhs: Box::new(Expr::Literal(Literal::Int(4,),)),
                                        }),
                                        rhs: Box::new(Expr::Literal(Literal::Int(4,),)),
                                    }),
                                    rhs: Box::new(Expr::BinaryOp {
                                        op: ast::BinaryOperator::Divide,
                                        lhs: Box::new(Expr::BinaryOp {
                                            op: ast::BinaryOperator::Multiply,
                                            lhs: Box::new(Expr::BinaryOp {
                                                op: ast::BinaryOperator::Multiply,
                                                lhs: Box::new(Expr::Literal(Literal::Int(4))),
                                                rhs: Box::new(Expr::Literal(Literal::Int(6)))
                                            }),
                                            rhs: Box::new(Expr::Literal(Literal::Int(20)))
                                        }),
                                        rhs: Box::new(Expr::Literal(Literal::Int(15)))
                                    })
                                }]
                            )
                        ]
                    }
                }]
            }
        )
    }
    #[test]
    fn parsing_match() {
        assert_eq!(
            parse(
                r#"
    // hi
    /*
     this too
    */
    main :: (args: [string]) {
        x :: true;
        match x 
        | true => println("this will always happen")
        | false => println("this will never happen")
        
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
                        body: vec![
                            Expr::Binding {
                                ident: "x".into(),
                                value: Box::new(Expr::Literal(Literal::Bool(true)))
                            },
                            Expr::Match {
                                target: Box::new(Expr::Ident("x".into())),
                                conditions: vec![
                                    (
                                        Literal::Bool(true),
                                        Expr::FunctionCall(
                                            Box::new(Expr::Ident("println".into())),
                                            vec![Expr::Literal(Literal::String(
                                                "this will always happen".into()
                                            ))]
                                        )
                                    ),
                                    (
                                        Literal::Bool(false),
                                        Expr::FunctionCall(
                                            Box::new(Expr::Ident("println".into())),
                                            vec![Expr::Literal(Literal::String(
                                                "this will never happen".into()
                                            ))]
                                        )
                                    )
                                ]
                            }
                        ]
                    }
                }]
            }
        )
    }
}
