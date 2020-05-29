// use crate::statements::*;
// use m68k_reloaded_common::errors::{Error, ErrorCollector};
// use m68k_reloaded_common::{Cursored, Range};
// use m68k_reloaded_scanner::Token;

// pub fn parse(tokens: Vec<Token>, errors: &mut ErrorCollector) -> Parser {
//     let tokens = tokens
//         .into_iter()
//         .filter(|token| !matches!(token, Token::Whitespace(_)))
//         .collect();
//     Parser::from_string(tokens, errors)
// }

// type Parser<'e> = CursorParser<'e>;

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use m68k_reloaded_scanner::scan;

//     #[test]
//     fn test_parse() {
//         let source = "MOVE.W D3, D6";
//         let mut errors = vec![];

//         let tokens: Vec<Token> = scan(source, &mut errors).collect();
//         let statements: Vec<Statement> = parse(&tokens, &mut errors).collect();
//         for statement in &statements {
//             println!("Got statement {:?}.", statement);
//         }
//     }
// }
