use std::cmp::Ordering;
use std::fmt::{self, Display};

#[derive(PartialEq, Eq, Debug)]
pub enum Severity {
    Info,
    Warning,
    Error,
}

impl Severity {
    fn ordinal(&self) -> u8 {
        match &self {
            Severity::Info => 1,
            Severity::Warning => 2,
            Severity::Error => 3,
        }
    }
}

impl Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match &self {
            Severity::Info => "info",
            Severity::Warning => "warning",
            Severity::Error => "error",
        })
    }
}

impl Ord for Severity {
    fn cmp(&self, other: &Self) -> Ordering {
        self.ordinal().cmp(&other.ordinal())
    }
}
impl PartialOrd for Severity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
