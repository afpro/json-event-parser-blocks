//! Utilities and serde support for the `json_event_parser` event stream.
//!
//! Provides the `Skipper` for skipping whole values, helper functions for
//! `JsonEvent`, and (with the `serde` feature) `JsonSerializer` / `JsonDeserializer`.

#[cfg(feature = "serde")]
mod serde;
mod skipper;

use json_event_parser::JsonEvent;
#[cfg(feature = "serde")]
pub use serde::{JsonEventSlice, JsonEventSource, JsonSerializer, JsonDeserializer, SerDeIoError};
pub use skipper::{SkipError, Skipper};

/// Returns the static variant name of a `JsonEvent` (e.g. `"StartObject"`).
pub fn event_name(event: &JsonEvent<'_>) -> &'static str {
    match event {
        JsonEvent::String(_) => "String",
        JsonEvent::Number(_) => "Number",
        JsonEvent::Boolean(_) => "Boolean",
        JsonEvent::Null => "Null",
        JsonEvent::StartArray => "StartArray",
        JsonEvent::EndArray => "EndArray",
        JsonEvent::StartObject => "StartObject",
        JsonEvent::EndObject => "EndObject",
        JsonEvent::ObjectKey(_) => "ObjectKey",
        JsonEvent::Eof => "Eof",
    }
}

/// Converts a borrowed `JsonEvent` into an owned `JsonEvent<'static>`.
///
/// Borrowed string/number payloads are copied into owned data so the result
/// no longer borrows from the input.
pub fn owned_event(event: JsonEvent<'_>) -> JsonEvent<'static> {
    match event {
        JsonEvent::String(v) => JsonEvent::String(v.into_owned().into()),
        JsonEvent::Number(v) => JsonEvent::Number(v.into_owned().into()),
        JsonEvent::Boolean(v) => JsonEvent::Boolean(v),
        JsonEvent::Null => JsonEvent::Null,
        JsonEvent::StartArray => JsonEvent::StartArray,
        JsonEvent::EndArray => JsonEvent::EndArray,
        JsonEvent::StartObject => JsonEvent::StartObject,
        JsonEvent::EndObject => JsonEvent::EndObject,
        JsonEvent::ObjectKey(v) => JsonEvent::ObjectKey(v.into_owned().into()),
        JsonEvent::Eof => JsonEvent::Eof,
    }
}
