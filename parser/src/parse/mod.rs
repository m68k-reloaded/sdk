use crate::statements::*;
use m68k_reloaded_common::errors::{Collector, Error};
use m68k_reloaded_common::Range;
use m68k_reloaded_scanner::Scanner;
use m68k_reloaded_scanner::Token;
use std::iter::Peekable;
use std::mem;

mod whitespace_remover;
use whitespace_remover::WithoutWhitespace;

type TokenIterator<'t> = dyn Iterator<Item = &'t Token>;

pub fn parse<'s, 't, 'e>(
    scanner: &'s mut Scanner<'s, 'e>,
    errors: &'e mut Collector,
) -> Parser<'s, 'e> {
    Parser {
        scanner,
        errors,
        buffer: vec![],
    }
}

pub struct Parser<'s, 'e> {
    scanner: &Scanner<'s, 'e>,
    errors: &'e mut Collector,
    buffer: Vec<Token>,
}

impl<'s> Parser<'s, '_> {
    fn is_at_end(&mut self) -> bool {
        self.scanner.peek() == None
    }

    fn flush(&mut self) {
        self.buffer.clear()
    }

    fn peek(&mut self) -> Option<&Token> {
        self.scanner.peek()
    }

    fn advance(&mut self) -> Token {
        self.scanner.next().unwrap()
    }

    fn advance_while<Test>(&mut self, test: Test) -> Vec<Token>
    where
        Test: Fn(&Token) -> bool,
    {
        while !self.is_at_end() && test(self.peek().unwrap()) {
            self.advance();
        }
        mem::replace(&mut self.buffer, vec![])
    }

    fn range(&self) -> Option<Range> {
        if self.buffer.is_empty() {
            None
        } else {
            let start = self.buffer.first().unwrap().range().start;
            let end = self.buffer.last().unwrap().range().end;
            Some(start..end)
        }
    }

    fn scan_next_statement(&mut self) -> Result<Stmt<Statement>, Error> {
        let token = match (self.advance(), self.peek()) {
            ('(', _) => Ok(Token::OpeningParen(self.range())),
            (')', _) => Ok(Token::ClosingParen(self.range())),
            (',', _) => Ok(Token::Comma(self.range())),
            ('.', _) => Ok(Token::Dot(self.range())),
            ('+', _) => Ok(Token::Plus(self.range())),
            ('#', _) => Ok(Token::NumberSign(self.range())),
            (':', _) => Ok(Token::Colon(self.range())),
            ('0'..='9', _) | ('-', '0'..='9') => self.parse_decimal_number(),
            ('$', _) => self.parse_hex_number(),
            ('-', _) => Ok(Token::Minus(self.range())),
            // TODO(marcelgarus): Merge the following branches into one as soon as or-patterns are supported.
            (';', _) => self.parse_comment(),
            ('*', _) => self.parse_comment(),
            // TODO(marcelgarus): Merge the following branches into one as soon as or-patterns are supported.
            (' ', _) => Ok(Token::Whitespace(self.range())),
            ('\t', _) => Ok(Token::Whitespace(self.range())),
            (' ', _) => Ok(Token::Whitespace(self.range())),
            ('\r', '\n') => {
                self.advance();
                Ok(Token::Newline(self.range()))
            }
            ('\n', _) => Ok(Token::Newline(self.range())),
            // TODO(marcelgarus): Merge the following branches into one as soon
            // as or-patterns are supported.
            ('a'..='z', _) => self.parse_identifier(),
            ('A'..='Z', _) => self.parse_identifier(),
            ('_', _) => self.parse_identifier(),
            (current, next) => Err(Error::no_match(self.range(), current, next)),
        };
        self.flush();
        token
    }
}

impl<'t, 'e> Iterator for Parser<'t, 'e> {
    // TODO(marcelgarus): implement
    type Item = Stmt<Statement>;

    fn next(&mut self) -> Option<Stmt<Statement>> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use m68k_reloaded_scanner::scan;

    #[test]
    fn test_parse() {
        let source = "MOVE.W D3, D6";
        let mut errors = vec![];

        let tokens: Vec<Token> = scan(source, &mut errors).collect();
        let statements: Vec<Statement> = parse(&tokens, &mut errors).collect();
        for statement in &statements {
            println!("Got statement {:?}.", statement);
        }
    }
}
