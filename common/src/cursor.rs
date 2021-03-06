use crate::errors::Error;
use crate::errors::ErrorCollector;

/// Provides a cursor-based parsing environment and error handling.
///
/// This class is used by multiple parsers on different abstraction layers. It offers two features:
/// Consuming the source items piece by piece and registering errors.
/// Internally, it keeps a vector of items and a cursor position. All of the items before the cursor
/// position are empty, while the items at or after the cursor position are guaranteed to be
/// non-empty. By advancing the `CursorParser`, items are moved out of the `CursorParser` structure
/// and the cursor is advanced. Contrary it standard `Iterator`s or `Peekable`s, the `CursorParser`
/// allows peeking arbitrarily beyond the cursor position (peeking beyond the bounds of the vector
/// just results in returning None).
pub struct CursorParser<'a, T: Clone> {
    items: Vec<Option<T>>,
    cursor: usize,
    error_registry: &'a mut ErrorCollector,
}

impl<T: Clone> CursorParser<'_, T> {
    /// Creates a new struct that contains the given items.
    pub fn from<'a>(items: Vec<T>, error_registry: &'a mut ErrorCollector) -> CursorParser<'a, T> {
        CursorParser {
            items: items.into_iter().map(Some).collect(),
            cursor: 0,
            error_registry,
        }
    }

    pub fn cursor(&self) -> usize {
        self.cursor
    }

    pub fn is_done(&self) -> bool {
        self.cursor < self.items.len()
    }

    /// Registers an error.
    pub fn register(&mut self, error: Error) {
        self.error_registry.push(error);
    }

    /// Returns the next item (the same that calling `advance()` would return).
    pub fn peek(&self) -> Option<T> {
        if self.cursor < self.items.len() {
            let maybe_item: Option<T> = self.items[self.cursor].clone();
            let item: T = maybe_item.unwrap();
            Some(item)
        } else {
            None
        }
    }

    pub fn peek_map<R, M>(&self, mapper: M) -> Option<R>
    where
        M: FnOnce(&T) -> R,
    {
        self.peek().map(|item| mapper(&item))
    }

    /// Moves the current item out of the structure and advances the cursor.
    pub fn advance(&mut self) -> Option<T> {
        self.cursor += 1;
        if self.cursor <= self.items.len() {
            std::mem::replace(&mut self.items[self.cursor - 1], None)
        } else {
            None
        }
    }

    /// Advances the cursor n times and returns the last item.
    pub fn advance_n(&mut self, n: usize) -> Vec<Option<T>> {
        let mut items: Vec<Option<T>> = vec![];
        for _ in 0..n {
            items.push(self.advance());
        }
        items
    }

    /// Expects the predicate to match the next character. If it does, consumes the next item and
    /// returns it. Otherwise returns `None`.
    pub fn advance_if<P>(&mut self, predicate: P) -> Option<T>
    where
        P: FnOnce(&T) -> bool,
    {
        match self.peek() {
            Some(item) if predicate(&item) => {
                let item = self.advance()?;
                Some(item)
            }
            _ => None,
        }
    }

    pub fn advance_if_map<S, P>(&mut self, predicate: P) -> Option<S>
    where
        P: FnOnce(T) -> Option<S>,
    {
        match self.peek() {
            None => None,
            Some(item) => match predicate(item) {
                Some(mapped_value) => Some(mapped_value),
                None => None,
            },
        }
    }

    /// Advances the cursor multiple times until the predicate returns `false`. Then, returns a
    /// vector containing all of the removed items.
    pub fn advance_while<P>(&mut self, predicate: P) -> Vec<T>
    where
        P: Fn(&T) -> bool,
    {
        self.advance_while_with_initial(predicate, vec![])
    }

    /// Like advance while but also accepts an initial vector that the removed items are added to.
    pub fn advance_while_with_initial<P>(&mut self, predicate: P, initial: Vec<T>) -> Vec<T>
    where
        P: Fn(&T) -> bool,
    {
        let mut elements = initial;
        loop {
            match self.advance_if(&predicate) {
                Some(item) => elements.push(item),
                None => break elements,
            }
        }
    }
}
