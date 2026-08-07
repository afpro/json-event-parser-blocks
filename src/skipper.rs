use json_event_parser::JsonEvent;
use std::fmt::Debug;

#[derive(Copy, Clone, Eq, PartialEq, Debug, thiserror::Error)]
pub enum SkipError {
    #[error("Skip already done")]
    SkipAlreadyDone,
    #[error("Stack is empty (trying to skip after done?)")]
    StackEmpty,
    #[error("Nested object key")]
    NestedObjectKey,
    #[error("Eof")]
    Eof,
}

#[derive(Copy, Clone)]
pub struct Skipper {
    has_skipped_value: bool,
    depth: usize,
}

impl Default for Skipper {
    #[inline]
    fn default() -> Self {
        Skipper::new()
    }
}

impl Skipper {
    #[inline]
    #[doc = "create new Skipper instance"]
    pub const fn new() -> Self {
        Self {
            has_skipped_value: false,
            depth: 0,
        }
    }

    #[inline]
    #[doc = "in skipping state (between object/array, or just start without any value skipped)"]
    pub fn skipping(&self) -> bool {
        !self.has_skipped_value || self.depth > 0
    }

    #[doc = "reset to init state, for reuse. (create new one is cheap, this is not necessary)"]
    pub fn reset(&mut self) {
        self.has_skipped_value = false;
        self.depth = 0;
    }

    #[doc = "feed new event to Skipper, return `skipping()`."]
    pub fn on_event(&mut self, event: &JsonEvent<'_>) -> Result<bool, SkipError> {
        if !self.skipping() {
            return Err(SkipError::SkipAlreadyDone);
        }

        match event {
            JsonEvent::String(_)
            | JsonEvent::Number(_)
            | JsonEvent::Boolean(_)
            | JsonEvent::Null => {
                self.has_skipped_value = true;
            }
            JsonEvent::StartArray | JsonEvent::StartObject => {
                self.has_skipped_value = true;
                self.depth += 1;
            }
            JsonEvent::EndArray | JsonEvent::EndObject => {
                if self.depth == 0 {
                    return Err(SkipError::StackEmpty);
                }
                self.depth -= 1;
            }
            JsonEvent::ObjectKey(_) => {
                if self.depth == 0 {
                    return Err(SkipError::NestedObjectKey);
                }
            }
            JsonEvent::Eof => {
                return Err(SkipError::Eof);
            }
        }

        Ok(self.skipping())
    }
}

