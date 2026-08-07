#[cfg(feature = "serde")]
mod serde;
mod skipper;

use json_event_parser::JsonEvent;
#[cfg(feature = "serde")]
pub use serde::{JsonEventSlice, JsonEventSource, JsonValueSink, JsonValueSource, SerDeIoError};
pub use skipper::{SkipError, Skipper};

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
