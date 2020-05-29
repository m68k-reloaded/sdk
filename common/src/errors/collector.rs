use super::Error;

#[derive(Default)]
pub struct Collector {
    errors: Vec<Error>,
}

impl Collector {
    pub fn push(&mut self, error: Error) {
        self.errors.push(error);
    }

    pub fn len(&self) -> usize {
        self.errors.len()
    }

    pub fn is_not_empty(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn print(&self) {
        for error in &self.errors {
            println!("Error at byte {}: {}", error.range.start, error.message);
        }
    }
}
