pub use m68k_reloaded_common::Range;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Token {
  // Single characters.
  OpeningParen(Range), // (
  ClosingParen(Range), // )
  Comma(Range),        // ,
  Dot(Range),          // .
  Minus(Range),        // -
  Plus(Range),         // +
  NumberSign(Range),   // #
  Colon(Range),        // :

  // Literals.
  Comment(Range, String),
  Identifier(Range, String),
  Number(Range, u32),

  // Whitespace.
  Whitespace(Range),
  Newline(Range),
}

impl Token {
  pub fn range(&self) -> Range {
    match self {
      Token::OpeningParen(range)
      | Token::ClosingParen(range)
      | Token::Comma(range)
      | Token::Dot(range)
      | Token::Minus(range)
      | Token::Plus(range)
      | Token::NumberSign(range)
      | Token::Colon(range)
      | Token::Comment(range, _)
      | Token::Identifier(range, _)
      | Token::Number(range, _)
      | Token::Whitespace(range)
      | Token::Newline(range) => range.clone(),
    }
  }
}

/*class Token {
  String toString() {
    return '${type.toString().substring('TokenType.'.length)} at $location: "$lexeme" (Literal: $literal)';
  }
}*/
