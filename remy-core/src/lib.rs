pub mod parser;

#[cfg(test)]
mod tests {
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
                Token::WhiteSpace,
                Token::LineComment,
                Token::WhiteSpace,
                Token::BlockComment,
                Token::WhiteSpace,
                Token::Ident("main"),
                Token::WhiteSpace,
                Token::Colon,
                Token::Colon,
                Token::WhiteSpace,
                Token::LParen,
                Token::Ident("args"),
                Token::Colon,
                Token::WhiteSpace,
                Token::LBracket,
                Token::Ident("string"),
                Token::RBracket,
                Token::RParen,
                Token::WhiteSpace,
                Token::FatArrow,
                Token::WhiteSpace,
                Token::LBrace,
                Token::WhiteSpace,
                Token::Ident("println"),
                Token::LParen,
                Token::NormalString("\"Hello to:\""),
                Token::RParen,
                Token::SemiColon,
                Token::WhiteSpace,
                Token::LineComment,
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
                Token::WhiteSpace,
                Token::RBrace,
                Token::WhiteSpace,
            ]
        );
    }
}
