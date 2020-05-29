use super::{super::Range, Error, Severity, Source};

impl Error {
    pub fn unspecified_size(range: Range) -> Error {
        Error {
            code: "unspecified_size",
            severity: Severity::Error,
            source: Source::Compiler,
            range,
            message: "A size attribute isn't present and could not be inferred.".to_string(),
        }
    }
}
