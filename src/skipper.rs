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
    pub const fn new() -> Self {
        Self {
            has_skipped_value: false,
            depth: 0,
        }
    }

    #[inline]
    pub fn skipping(&self) -> bool {
        !self.has_skipped_value || self.depth > 0
    }

    pub fn reset(&mut self) {
        self.has_skipped_value = false;
        self.depth = 0;
    }

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

