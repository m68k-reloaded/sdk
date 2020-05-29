use crate::Range;
pub use collector::Collector;
pub use severity::Severity;

mod collector;
pub mod compiler;
pub mod scanner;
mod severity;

pub struct Error {
    pub code: &'static str,
    pub severity: Severity,
    pub source: Source,
    pub range: Range,
    pub message: String,
}

#[derive(PartialEq, Eq, Debug)]
pub enum Source {
    Scanner,
    Parser,
    Compiler,
}
