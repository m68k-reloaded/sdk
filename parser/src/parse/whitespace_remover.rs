use m68k_reloaded_scanner::Token;
use std::iter::Filter;

trait Squarable {
    fn without_whitespace(&mut self) -> Box<dyn Iterator<Item = Token>>;
}

impl<I: Iterator<Item = Token>> Squarable for I {
    fn without_whitespace(&mut self) -> Box<dyn Iterator<Item = Token>> {
        Box::new(self.filter(|token| match token {
            Token::Whitespace(_) => false,
            _ => true,
        }))
    }
}

type Filtered<'a, I: Iterator<Item = &'a Token>> = Filter<&'a mut I, dyn Fn(&'a Token) -> bool>;

pub trait WithoutWhitespace<I> {
    fn without_whitespace(&mut self) -> Filtered<I>;
}

impl<'t, I: Iterator<Item = &'t Token>> WithoutWhitespace<&mut I> for &mut I {
    fn without_whitespace(&'t mut self) -> Filtered<'t, I> {
        fn filter(token: &'t Token) -> bool {
            match token {
                Token::Whitespace(_) => false,
                _ => true,
            }
        };
        let filtered = self.filter(filter);
        filtered
    }
}
