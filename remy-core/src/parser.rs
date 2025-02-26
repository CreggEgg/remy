use std::ops::Range;

use chumsky::{
    error::Simple,
    primitive::{any, choice, end, filter, filter_map, just},
    recursive::recursive,
    select, Parser,
};
use logos::{Lexer, Logos, Skip, Span};

use crate::ast::{
    AnnotatedIdent, BinaryOperator, BindingLeftHand, ConstrainedType, Expr, File, Ident, Literal,
    TopLevelDefinition, TypeName,
};
#[derive(Debug)]
pub enum ParseError<'a> {
    UnrecognizedToken(Range<usize>),
    ParseFailed(Vec<Simple<Token<'a>>>),
}

pub fn parse(file_to_parse: &str) -> Result<File, ParseError<'_>> {
    let mut tokens = vec![];
    let mut lexer = Token::lexer(file_to_parse);
    while let Some(tok) = lexer.next() {
        tokens.push(match tok {
            Ok(tok) => Ok(/* Spanned( */ tok /* , lexer.span()) */),
            Err(_) => Err(ParseError::UnrecognizedToken(lexer.span())),
        }?);
    }
    dbg!(&tokens);
    file()
        .parse(tokens)
        .map_err(|err| ParseError::ParseFailed(err)) //.replace_err()
}
#[derive(Debug, PartialEq, Default)]
pub(crate) enum StringState {
    #[default]
    NotStarted,
    Started,
}

#[derive(Debug, PartialEq, Default)]
pub struct LogosState {
    string_state: StringState,
}

#[derive(Logos, Debug, PartialEq, Eq, Hash, Clone)]
#[logos(skip "(//[^\n]+)|([ \t\r\n\n]+)|(/\\*[^(*/)]+\\*/)")]
#[logos(extras = LogosState)]
pub enum Token<'a> {
    // #[regex("//[^\n]*")]
    // LineComment,
    // #[regex("[ \t\r\n\n]+")]
    // WhiteSpace,
    // #[regex(r#"/\*[^(*/)]*\*/"#)]
    // BlockComment,
    #[token(",")]
    Comma,
    #[token("::", priority = 2)]
    DoubleColon,
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
    #[token("&")]
    And,
    #[token("{")]
    LBrace,
    #[token("}", priority = 1)]
    RBrace,
    #[token("<")]
    LessThan,
    #[token(">")]
    GreaterThan,
    #[regex(r#"[0-9]+(\.(0-9)+)?"#)]
    Number(&'a str),
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Multiply,
    #[token("/")]
    Divide,
    #[token("|")]
    Bar,
    #[token("match")]
    Match,
    #[token("extern")]
    Extern,
}
#[derive(Clone)]
pub struct Spanned<T: Clone>(pub T, pub Span);

pub(crate) fn file<'a>() -> impl Parser<Token<'a>, File, Error = Simple<Token<'a>>> {
    // any().map(|_| File {
    //     definitions: vec![],
    // })

    top_level_definition()
        .repeated()
        .then_ignore(end())
        .map(|definitions| File { definitions })
}
fn top_level_definition<'a>(
) -> impl Parser<Token<'a>, TopLevelDefinition, Error = Simple<Token<'a>>> {
    choice((binding_definition(), binding_external()))
}
fn binding_definition<'a>() -> impl Parser<Token<'a>, TopLevelDefinition, Error = Simple<Token<'a>>>
{
    binding_left_hand()
        // .padded_by(ws())
        .then_ignore(just(Token::DoubleColon))
        .then(literal())
        .map(|(binding_left_hand, value)| TopLevelDefinition::Binding {
            lhs: binding_left_hand,
            rhs: value,
        })
}
fn binding_external<'a>() -> impl Parser<Token<'a>, TopLevelDefinition, Error = Simple<Token<'a>>> {
    just(Token::Extern)
        .ignore_then(ident())
        // .padded_by(ws())
        .then_ignore(just(Token::DoubleColon))
        .then(type_name())
        .map(|(name, type_name)| TopLevelDefinition::Extern {
            name,
            rhs: type_name,
        })
}

fn binding_left_hand<'a>() -> impl Parser<Token<'a>, BindingLeftHand, Error = Simple<Token<'a>>> {
    ident()
        .then(
            just(Token::LessThan)
                .ignore_then(constrained_type().separated_by(just(Token::Comma)))
                .then_ignore(just(Token::GreaterThan))
                .or_not(),
        )
        .map(|(name, type_args)| BindingLeftHand {
            name,
            type_args: type_args.unwrap_or_default(),
        })
}
fn constrained_type<'a>() -> impl Parser<Token<'a>, ConstrainedType, Error = Simple<Token<'a>>> {
    ident()
        .then(
            (just(Token::Colon))
                .ignore_then(ident().separated_by(just(Token::And)))
                .or_not(),
        )
        .map(|(name, constraints)| ConstrainedType {
            name,
            constraints: constraints.unwrap_or_default(),
        })
}

fn literal<'a>() -> impl Parser<Token<'a>, Literal, Error = Simple<Token<'a>>> {
    choice((string_literal(), function_literal()))
}
fn function_literal<'a>() -> impl Parser<Token<'a>, Literal, Error = Simple<Token<'a>>> {
    annotated_ident()
        .separated_by(just(Token::Comma))
        .delimited_by(just(Token::LParen), just(Token::RParen))
        .then(type_name().or_not())
        .then(code_block())
        .map(|((args, ret_type), body)| Literal::Function {
            args,
            ret_type: ret_type.unwrap_or(TypeName::Named("unit".into())),
            body,
        })
}
fn code_block<'a>() -> impl Parser<Token<'a>, Vec<Expr>, Error = Simple<Token<'a>>> {
    expr()
        .separated_by(just(Token::SemiColon))
        .delimited_by(just(Token::LBrace), just(Token::RBrace))
}
fn expr<'a>() -> impl Parser<Token<'a>, Expr, Error = Simple<Token<'a>>> {
    recursive(|expr| {
        // let function_call = expr
        //     .clone()
        //     .then(
        //         expr.clone()
        //             .separated_by(just(Token::Comma))
        //             .delimited_by(just(Token::LParen), just(Token::RParen)),
        //     )
        //     .map(|(fun, args)| Expr::FunctionCall(Box::new(fun), args));
        //
        //
        //
        let code_block = expr
            .clone()
            .separated_by(just(Token::SemiColon))
            .delimited_by(just(Token::LBrace), just(Token::RBrace));

        let function_literal = annotated_ident()
            .separated_by(just(Token::Comma))
            .delimited_by(just(Token::LParen), just(Token::RParen))
            .then(type_name().or_not())
            .then(code_block)
            .map(|((args, ret_type), body)| Literal::Function {
                args,
                ret_type: ret_type.unwrap_or(TypeName::Named("unit".into())),
                body,
            });

        let literal = choice((
            string_literal(),
            bool_literal(),
            int_literal(),
            function_literal,
        )); //.labelled("Literal");

        let r#match = just(Token::Match)
            .ignore_then(expr.clone())
            .then(
                just(Token::Bar)
                    .ignore_then(literal.clone())
                    .then_ignore(just(Token::FatArrow))
                    .then(expr.clone())
                    .repeated()
                    .at_least(1),
            )
            .map(|(condition, cases)| Expr::Match {
                target: Box::new(condition),
                conditions: cases,
            });
        //     .then(
        //         just(Token::Bar)
        //             .ignore_then(literal())
        //             .then_ignore(just(Token::FatArrow))
        //             // .then(expr.clone())
        //             .repeated()
        //             .at_least(1),
        //     )
        //     .map(|(condition, cases)| Expr::Match {
        //         target: condition,
        //         conditions: vec![],
        //     });

        // .labelled("Code block");
        let binding = ident()
            .then_ignore(just(Token::DoubleColon))
            .then(expr.clone())
            .map(|(ident, value)| Expr::Binding {
                ident,
                value: Box::new(value),
            });

        let function_call = ident()
            .then(
                expr.separated_by(just(Token::Comma))
                    .delimited_by(just(Token::LParen), just(Token::RParen)),
            )
            .map(|(f, args)| Expr::FunctionCall(Box::new(Expr::Ident(f)), args));

        let atom = choice((
            literal.map(|lit| Expr::Literal(lit)),
            binding,
            function_call,
            r#match,
            ident().map(|i| Expr::Ident(i)),
        ));
        // .labelled("Atom");
        let unary = atom.clone();

        let product = unary
            .clone()
            // .clone()
            .then(
                (select! {
                    Token::Multiply => BinaryOperator::Multiply,
                    Token::Divide => BinaryOperator::Divide,
                }
                .then(unary))
                .repeated(),
            )
            .foldl(|lhs, (op, rhs)| Expr::BinaryOp {
                op,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            });
        // .map(|(lhs, rhs)| match rhs {
        //     Some((op, rhs)) => Expr::BinaryOp {
        //         op,
        //         lhs: Box::new(lhs),
        //         rhs: Box::new(rhs),
        //     },
        //     None => lhs,
        // });
        let sum = product
            .clone()
            .then(
                (select! {
                    Token::Plus => BinaryOperator::Add,
                    Token::Minus => BinaryOperator::Subtract,
                }
                .then(product))
                .repeated(),
            )
            .foldl(|lhs, (op, rhs)| Expr::BinaryOp {
                op,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            });
        // .map(|(lhs, rhs)| {
        //     let mut rhs_final =
        // });
        sum
    })
}

fn annotated_ident<'a>() -> impl Parser<Token<'a>, AnnotatedIdent, Error = Simple<Token<'a>>> + Clone
{
    ident()
        .then_ignore(just(Token::Colon))
        .then(type_name())
        .map(|(ident, annotation)| AnnotatedIdent {
            name: ident,
            r#type: annotation,
        })
}

fn type_name<'a>() -> impl Parser<Token<'a>, TypeName, Error = Simple<Token<'a>>> + Clone {
    recursive(|type_name| {
        choice((
            ident().map(|name| TypeName::Named(name)),
            type_name
                .clone()
                .delimited_by(just(Token::LBracket), just(Token::RBracket))
                .map(|name| TypeName::Slice(Box::new(name))),
            type_name
                .clone()
                .separated_by(just(Token::Comma))
                .delimited_by(just(Token::LParen), just(Token::RParen))
                // .then_ignore(just(Token::Colon))
                .then(type_name)
                .map(|(args, ret)| TypeName::Function {
                    args,
                    ret: Box::new(ret),
                }),
        ))
    })
}

fn string_literal<'a>() -> impl Parser<Token<'a>, Literal, Error = Simple<Token<'a>>> + Clone {
    select! {
        Token::NormalString(s) => Literal::String(s[1..s.len() - 1].into())
    }
}
fn int_literal<'a>() -> impl Parser<Token<'a>, Literal, Error = Simple<Token<'a>>> + Clone {
    select! {
        Token::Number(s) => match s.parse::<i64>() {
            Ok(num) => Literal::Int(num),
            Err(_) => Literal::Float(s.parse().unwrap()),
        }
    }
}
fn bool_literal<'a>() -> impl Parser<Token<'a>, Literal, Error = Simple<Token<'a>>> + Clone {
    select! {
        Token::Ident("true") => Literal::Bool(true),
        Token::Ident("false") => Literal::Bool(false),

    }
}

// fn ws<'a>() -> impl Parser<Token<'a>, (), Error = Simple<Token<'a>>> {
//     choice((
//         just(Token::WhiteSpace),
//         just(Token::LineComment),
//         just(Token::BlockComment),
//     ))
//     .repeated()
//     .ignored()
// }
fn ident<'a>() -> impl Parser<Token<'a>, Ident, Error = Simple<Token<'a>>> + Clone {
    filter(|t| matches!(t, Token::Ident(_))).map(|t| {
        if let Token::Ident(i) = t {
            i.into()
        } else {
            unreachable!()
        }
    })
}
