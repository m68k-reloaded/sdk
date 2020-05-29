use super::Error;

pub type ErrorCollector = Vec<Error>;

pub trait PrintErrors {
    fn print(&self);
}

impl PrintErrors for ErrorCollector {
    fn print(&self) {
        unimplemented!()
    }
}
