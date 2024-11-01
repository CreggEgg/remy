use chumsky::{
    error::Simple,
    primitive::{any, choice, end, filter, filter_map, just},
    recursive::recursive,
    select, Parser,
};
use logos::{Lexer, Logos, Skip, Span};

use crate::ast::{AnnotatedIdent, Expr, File, Ident, Literal, TopLevelDefinition, TypeName};
#[derive(Debug, PartialEq, Default)]
pub(crate) enum StringState {
    #[default]
    NotStarted,
    Started,
}

#[derive(Debug, PartialEq, Default)]
pub(crate) struct LogosState {
    string_state: StringState,
}

#[derive(Logos, Debug, PartialEq, Eq, Hash, Clone)]
#[logos(skip "(//[^\n]*)|([ \t\r\n\n]+)|(/\\*[^(*/)]*\\*/)")]
#[logos(extras = LogosState)]
pub(crate) enum Token<'a> {
    // #[regex("//[^\n]*")]
    // LineComment,
    // #[regex("[ \t\r\n\n]+")]
    // WhiteSpace,
    // #[regex(r#"/\*[^(*/)]*\*/"#)]
    // BlockComment,
    #[token(",")]
    Comma,
    #[token("::")]
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
    #[token("{")]
    LBrace,
    #[token("}", priority = 1)]
    RBrace,
    #[regex(r#"[0-9]+(\.(0-9)+)?"#)]
    Number(&'a str),
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
    ident()
        // .padded_by(ws())
        .then_ignore(just(Token::DoubleColon))
        .then(literal())
        .map(|(name, value)| TopLevelDefinition::Binding { name, rhs: value })
}

fn literal<'a>() -> impl Parser<Token<'a>, Literal, Error = Simple<Token<'a>>> {
    choice((function_literal(), string_literal()))
}
fn function_literal<'a>() -> impl Parser<Token<'a>, Literal, Error = Simple<Token<'a>>> {
    annotated_ident()
        .separated_by(just(Token::Comma))
        .delimited_by(just(Token::LParen), just(Token::RParen))
        .then(code_block())
        .map(|(args, body)| Literal::Function { args, body })
}
fn code_block<'a>() -> impl Parser<Token<'a>, Vec<Expr>, Error = Simple<Token<'a>>> {
    expr()
        .separated_by(just(Token::SemiColon))
        .delimited_by(just(Token::LBracket), just(Token::RBracket))
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
        choice((
            ident().map(|i| Expr::Ident(i)),
            literal().map(|lit| Expr::Literal(lit)),
            // function_call,
        ))
    })
}
fn annotated_ident<'a>() -> impl Parser<Token<'a>, AnnotatedIdent, Error = Simple<Token<'a>>> {
    ident()
        .then(type_name())
        .map(|(ident, annotation)| AnnotatedIdent {
            name: ident,
            r#type: annotation,
        })
}

fn type_name<'a>() -> impl Parser<Token<'a>, TypeName, Error = Simple<Token<'a>>> {
    recursive(|type_name| {
        choice((
            ident().map(|name| TypeName::Named(name)),
            type_name
                .delimited_by(just(Token::LBrace), just(Token::RBrace))
                .map(|name| TypeName::Slice(Box::new(name))),
        ))
    })
}

fn string_literal<'a>() -> impl Parser<Token<'a>, Literal, Error = Simple<Token<'a>>> {
    select! {
        Token::NormalString(s) => Literal::String(s.into())
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

fn ident<'a>() -> impl Parser<Token<'a>, Ident, Error = Simple<Token<'a>>> {
    filter(|t| matches!(t, Token::Ident(_))).map(|t| {
        if let Token::Ident(i) = t {
            i.into()
        } else {
            unreachable!()
        }
    })
}
#[cfg(test)]
mod tests {
    use chumsky::{primitive::end, Parser};

    use crate::parser::Token;
}
