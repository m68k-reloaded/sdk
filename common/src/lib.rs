pub mod errors;

pub type Byte = u8;
pub type Word = u16;
pub type LongWord = u32;

/// Ranges have a start and end value.
pub type Range = std::ops::Range<usize>;

/// Locations have a line, column, and length.
pub struct Location {
    pub line: u8,
    pub column: u8,
    pub length: u8,
}

impl Location {
    pub fn invalid() -> Location {
        Location {
            line: 0,
            column: 0,
            length: 0,
        }
    }

    pub fn is_invalid(&self) -> bool {
        self.line == 0 || self.column == 0
    }
}
