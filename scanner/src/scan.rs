use crate::token::{Range, Token};
use m68k_reloaded_common::errors::{Error, ErrorCollector};

pub fn scan<'s, 'e>(source: &'s str, errors: &'e mut ErrorCollector) -> Scanner<'s, 'e> {
    Scanner {
        offset: 0,
        rest: source,
        cursor: 0,
        errors,
    }
}

pub struct Scanner<'s, 'e> {
    /// The rest of the original source code.
    rest: &'s str,
    /// The offset to the start of the original source.
    offset: usize,
    /// The cursor relative to the offset.
    cursor: usize,
    errors: &'e mut ErrorCollector,
}

impl<'s> Scanner<'s, '_> {
    fn is_at_end(&self) -> bool {
        self.rest[self.cursor..].is_empty()
    }

    fn flush(&mut self) {
        self.offset += self.cursor;
        self.rest = &self.rest[self.cursor..];
        self.cursor = 0;
    }

    fn lexeme(&self) -> String {
        self.rest.chars().take(self.cursor).collect()
    }

    fn peek(&self) -> char {
        match self.rest.chars().nth(self.cursor) {
            Some(character) => character,
            None => '\0',
        }
    }

    fn advance(&mut self) -> char {
        let removed = self.peek();
        self.cursor += 1;
        removed
    }

    fn advance_while<Test>(&mut self, test: Test) -> String
    where
        Test: Fn(char) -> bool,
    {
        while !self.is_at_end() && test(self.peek()) {
            self.advance();
        }
        self.lexeme()
    }

    fn range(&self) -> Range {
        self.offset..self.offset + self.cursor
    }

    fn scan_next_token(&mut self) -> Result<Token, Error> {
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

    fn parse_decimal_number(&mut self) -> Result<Token, Error> {
        let number = self.advance_while(|c| ('0'..'9').contains(&c));
        match number.parse() {
            Ok(number) => Ok(Token::Number(self.range(), number)),
            Err(_) => Err(Error::cannot_parse_decimal_number(self.range())),
        }
    }

    fn parse_hex_number(&mut self) -> Result<Token, Error> {
        let number = self.advance_while(|c| ('0'..'9').contains(&c) || ('a'..'f').contains(&c));
        match u32::from_str_radix(&number, 16) {
            Ok(number) => Ok(Token::Number(self.range(), number)),
            Err(_) => Err(Error::cannot_parse_hex_number(self.range())),
        }
    }

    fn parse_comment(&mut self) -> Result<Token, Error> {
        let content = self.advance_while(|c| c != '\n' && c != '\r');
        Ok(Token::Comment(self.range(), content))
    }

    fn parse_identifier(&mut self) -> Result<Token, Error> {
        let identifier = self.advance_while(|c| {
            ('a'..='z').contains(&c)
                || ('A'..='Z').contains(&c)
                || ('0'..='9').contains(&c)
                || c == '_'
        });
        Ok(Token::Identifier(self.range(), identifier))
    }

    // pub fn peek_token(&mut self)
}

impl Iterator for Scanner<'_, '_> {
    type Item = Token;

    fn next(&mut self) -> std::option::Option<Token> {
        while !self.is_at_end() {
            match self.scan_next_token() {
                Ok(token) => return Some(token),
                Err(error) => self.errors.push(error),
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_scan_empty_string() {
        expect_scanned_tokens("", vec![]);
    }
    #[test]
    fn test_scan_single_space() {
        expect_scanned_tokens(" ", vec![&Token::Whitespace(0..1)]);
    }
    #[test]
    fn test_scan_multiple_spaces() {
        expect_scanned_tokens(
            " \t",
            vec![&Token::Whitespace(0..1), &Token::Whitespace(1..2)],
        );
    }
    #[test]
    fn test_scan_empty_lines() {
        expect_scanned_tokens("\n\r\n", vec![&Token::Newline(0..1), &Token::Newline(1..3)]);
    }

    // TODO(marcelgarus): implement when we use line:col i/o Range
    // #[test]
    // fn test_correct_line_counting() {
    //     expect_scanned_tokens("*1\n*2\r*3\r\n*4", vec![Token::Comment(Range(), "1")]);
    // }

    #[test]
    fn test_scan_single_token() {
        let mut tokens: HashMap<&str, Token> = HashMap::new();

        tokens.insert("(", Token::OpeningParen(0..1));
        tokens.insert(")", Token::ClosingParen(0..1));
        tokens.insert(",", Token::Comma(0..1));
        tokens.insert(".", Token::Dot(0..1));
        tokens.insert("-", Token::Minus(0..1));
        tokens.insert("+", Token::Plus(0..1));
        tokens.insert("#", Token::NumberSign(0..1));
        tokens.insert(":", Token::Colon(0..1));

        for (source, expected) in tokens.iter() {
            expect_scanned_tokens(source, vec![expected]);
        }
    }

    #[test]
    fn test_scan_comment_empty() {
        expect_scanned_tokens("*", vec![&Token::Comment(0..1, String::from("*"))]);
        expect_scanned_tokens(";", vec![&Token::Comment(0..1, String::from(";"))]);
    }
    #[test]
    fn test_scan_comment_simple() {
        let comment = "*comment...";
        expect_scanned_tokens(comment, vec![&Token::Comment(0..11, String::from(comment))]);
    }
    #[test]
    fn test_scan_comment_unicode() {
        let comment = "äöüß é¡™£¢∞§¶•ªº–≠製漢語 ด้้้้้็็็็็้้้้้็็็็็้้้้้้้้็็็็็้้้้้็็็็็้้้้้้้้็็็็็้้้้้็็็็็้้้้้้้้็็็็็้้้้้็็็็❤️🇺🇸🇷🇺🇸 Ṱ̺̺̕o͞ ̷i̲̬͇̪͙n̝̗͕v̟̜̘̦͟o̶̙̰̠kè͚̮̺̪̹̱̤ ᴉlɐ";
        expect_scanned_tokens(comment, vec![&Token::Comment(0..11, String::from(comment))]);
    }

    #[test]
    fn test_scan_identifier() {
        let identifiers = vec![
        "i",
        "id",
        "_id",
        "id123",
        "D0",
        "D7",
        "A0",
        "A7",
        "Loremipsumdolorsitametconsecteturadipiscingelit_Maurisvitaeerosblanditipsumviverraposuereetanibh_Curabiturnislmetuslaciniautmagnaultricieselementumtempormassa",
        ];

        for identifier in identifiers.iter() {
            expect_scanned_tokens(
                identifier,
                vec![&Token::Identifier(
                    0..identifier.len(),
                    String::from(*identifier),
                )],
            );
        }
    }

    fn expect_scanned_tokens(source: &str, expected_tokens: Vec<&Token>) {
        let mut errors: Vec<Error> = Default::default();
        let tokens: Vec<Token> = scan(source, &mut errors).collect();

        errors.print();
        assert!(errors.is_empty());
        assert_eq!(tokens.len(), expected_tokens.len());
        for (actual, expected) in tokens.iter().zip(expected_tokens.iter()) {
            assert_eq!(&actual, expected);
        }
    }
}
