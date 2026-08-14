use json_event_parser::JsonEvent;
use std::fmt::Debug;

/// Errors that can occur while feeding events to a `Skipper`.
#[derive(Copy, Clone, Eq, PartialEq, Debug, thiserror::Error)]
pub enum SkipError {
    /// `on_event` was called after the target value had already been fully skipped.
    #[error("Skip already done")]
    SkipAlreadyDone,
    /// An `EndArray`/`EndObject` arrived without a matching open container.
    #[error("Stack is empty (trying to skip after done?)")]
    StackEmpty,
    /// An `ObjectKey` arrived at depth 0 (not inside an object being skipped).
    #[error("Nested object key")]
    NestedObjectKey,
    /// The event stream ended before the skip was complete.
    #[error("Eof")]
    Eof,
}

/// State machine that tracks whether a whole JSON value has been skipped.
///
/// Feed it events with [`Skipper::on_event`] until [`Skipper::skipping`] returns
/// `false`, which means the value currently being skipped is complete.
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
    /// Creates a new `Skipper` instance in the initial state.
    #[inline]
    pub const fn new() -> Self {
        Self {
            has_skipped_value: false,
            depth: 0,
        }
    }

    /// Returns whether the skipper is still in a skipping state.
    ///
    /// This is `true` either before any value has been skipped, or while inside
    /// a nested object/array whose end has not been reached yet.
    #[inline]
    pub fn skipping(&self) -> bool {
        !self.has_skipped_value || self.depth > 0
    }

    /// Resets the skipper to its initial state for reuse.
    ///
    /// Note: creating a new `Skipper` is cheap, so this is usually not necessary.
    pub fn reset(&mut self) {
        self.has_skipped_value = false;
        self.depth = 0;
    }

    /// Feeds a new event to the skipper and returns the updated [`Skipper::skipping`] state.
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

