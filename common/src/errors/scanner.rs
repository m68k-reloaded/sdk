use super::{super::Range, Error, Severity, Source};

impl Error {
    pub fn no_match(range: Range, current: char, next: char) -> Error {
        Error {
            code: "no_match",
            severity: Severity::Error,
            source: Source::Scanner,
            range,
            message: format!("No token matches for '{}', '{}'.", current, next),
        }
    }

    pub fn cannot_parse_decimal_number(range: Range) -> Error {
        Error {
            code: "cannot_parse_decimal_number",
            severity: Severity::Error,
            source: Source::Scanner,
            range,
            message: "Cannot parse the decimal number.".to_string(),
        }
    }

    pub fn cannot_parse_hex_number(range: Range) -> Error {
        Error {
            code: "cannot_parse_hex_number",
            severity: Severity::Error,
            source: Source::Scanner,
            range,
            message: "Cannot parse the hexadecimal number.".to_string(),
        }
    }
}
