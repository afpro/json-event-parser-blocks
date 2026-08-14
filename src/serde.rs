#![cfg(feature = "serde")]

use std::{error::Error, fmt::Display};

pub use de::{JsonEventSlice, JsonEventSource, JsonDeserializer};
use json_event_parser::JsonParseError;
pub use ser::JsonSerializer;
use serde::{de::Error as DeError, ser::Error as SerError};

/// The error type used by the serde `Serializer`/`Deserializer` implementations in this crate.
///
/// Wraps IO errors, JSON parse errors, and arbitrary custom errors.
#[derive(Debug, thiserror::Error)]
pub enum SerDeIoError {
    /// An underlying IO error.
    #[error("io error {}", _0)]
    Io(
        #[source]
        #[from]
        std::io::Error,
    ),
    /// A JSON parse error from `json_event_parser`.
    #[error("json parse error {}", _0)]
    Parse(
        #[source]
        #[from]
        JsonParseError,
    ),
    /// A custom error built from an arbitrary boxed error.
    #[error("custom error {}", _0)]
    Custom(
        #[source]
        #[from]
        Box<dyn Error + Send + Sync>,
    ),
}

impl SerDeIoError {
    /// Creates a `SerDeIoError` from an IO error kind and an arbitrary error message.
    #[inline]
    pub fn new<E>(kind: std::io::ErrorKind, msg: E) -> Self
    where
        E: Into<Box<dyn Error + Send + Sync>>,
    {
        std::io::Error::new(kind, msg.into()).into()
    }

    /// Creates a `SerDeIoError` representing unexpected end of input.
    #[inline]
    pub fn eof() -> Self {
        Self::new(std::io::ErrorKind::UnexpectedEof, "eof")
    }

    /// Creates a `SerDeIoError` from an arbitrary custom error.
    #[inline]
    pub fn custom<E>(msg: E) -> Self
    where
        E: Into<Box<dyn Error + Send + Sync>>,
    {
        msg.into().into()
    }
}

impl SerError for SerDeIoError {
    #[inline]
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self::custom(msg.to_string())
    }
}

impl DeError for SerDeIoError {
    #[inline]
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self::custom(msg.to_string())
    }
}

/// Serialization: `serde::Serialize` -> `JsonSerializer` -> `json_event_parser::WriterJsonSerializer`.
mod ser {
    use std::{borrow::Cow, io::Write};

    use json_event_parser::{JsonEvent, WriterJsonSerializer};
    use serde::{
        Serialize, Serializer,
        ser::{
            SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
            SerializeTupleStruct, SerializeTupleVariant,
        },
    };

    use super::SerDeIoError;

    /// serialize a single json value into output stream
    /// `serde::Serialize` -> `JsonSerializer` -> `json_event_parser::WriterJsonSerializer`
    pub struct JsonSerializer<'a, W: Write> {
        writer: &'a mut WriterJsonSerializer<W>,
    }

    impl<'a, W: Write> JsonSerializer<'a, W> {
        /// Creates a serializer wrapping the given `WriterJsonSerializer`.
        pub fn new(writer: &'a mut WriterJsonSerializer<W>) -> Self {
            Self { writer }
        }
    }

    impl<'a, W: Write> Serializer for JsonSerializer<'a, W> {
        type Ok = &'a mut WriterJsonSerializer<W>;
        type Error = SerDeIoError;
        type SerializeSeq = Self;
        type SerializeTuple = Self;
        type SerializeTupleStruct = Self;
        type SerializeTupleVariant = Self;
        type SerializeMap = Self;
        type SerializeStruct = Self;
        type SerializeStructVariant = Self;

        fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
            self.writer.serialize_event(JsonEvent::Boolean(v))?;
            Ok(self.writer)
        }

        fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
            self.writer
                .serialize_event(JsonEvent::Number(Cow::Owned(v.to_string())))?;
            Ok(self.writer)
        }

        fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
            self.writer
                .serialize_event(JsonEvent::Number(Cow::Owned(v.to_string())))?;
            Ok(self.writer)
        }

        fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
            self.writer
                .serialize_event(JsonEvent::Number(Cow::Owned(v.to_string())))?;
            Ok(self.writer)
        }

        fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
            self.writer
                .serialize_event(JsonEvent::Number(Cow::Owned(v.to_string())))?;
            Ok(self.writer)
        }

        fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
            self.writer
                .serialize_event(JsonEvent::Number(Cow::Owned(v.to_string())))?;
            Ok(self.writer)
        }

        fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
            self.writer
                .serialize_event(JsonEvent::Number(Cow::Owned(v.to_string())))?;
            Ok(self.writer)
        }

        fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
            self.writer
                .serialize_event(JsonEvent::Number(Cow::Owned(v.to_string())))?;
            Ok(self.writer)
        }

        fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
            self.writer
                .serialize_event(JsonEvent::Number(Cow::Owned(v.to_string())))?;
            Ok(self.writer)
        }

        fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
            self.writer
                .serialize_event(JsonEvent::Number(Cow::Owned(v.to_string())))?;
            Ok(self.writer)
        }

        fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
            self.writer
                .serialize_event(JsonEvent::Number(Cow::Owned(v.to_string())))?;
            Ok(self.writer)
        }

        fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
            self.writer
                .serialize_event(JsonEvent::String(Cow::Owned(v.to_string())))?;
            Ok(self.writer)
        }

        fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
            self.writer
                .serialize_event(JsonEvent::String(Cow::Borrowed(v)))?;
            Ok(self.writer)
        }

        // Serializes a byte slice as a JSON array of numbers.
        fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
            self.writer.serialize_event(JsonEvent::StartArray)?;
            self.writer.serialize_event(JsonEvent::EndArray)?;
            for b in v {
                self.writer
                    .serialize_event(JsonEvent::Number(Cow::Owned(b.to_string())))?;
            }
            Ok(self.writer)
        }

        fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
            self.writer.serialize_event(JsonEvent::Null)?;
            Ok(self.writer)
        }

        fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
        where
            T: ?Sized + Serialize,
        {
            value.serialize(self)
        }

        fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
            self.writer.serialize_event(JsonEvent::Null)?;
            Ok(self.writer)
        }

        fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
            self.serialize_unit()
        }

        fn serialize_unit_variant(
            self,
            _name: &'static str,
            _variant_index: u32,
            variant: &'static str,
        ) -> Result<Self::Ok, Self::Error> {
            self.serialize_str(variant)
        }

        fn serialize_newtype_struct<T>(
            self,
            _name: &'static str,
            value: &T,
        ) -> Result<Self::Ok, Self::Error>
        where
            T: ?Sized + Serialize,
        {
            value.serialize(self)
        }

        fn serialize_newtype_variant<T>(
            self,
            _name: &'static str,
            _variant_index: u32,
            variant: &'static str,
            value: &T,
        ) -> Result<Self::Ok, Self::Error>
        where
            T: ?Sized + Serialize,
        {
            let writer = self.writer;
            writer.serialize_event(JsonEvent::StartObject)?;
            writer.serialize_event(JsonEvent::ObjectKey(Cow::Borrowed(variant)))?;
            let writer = value.serialize(Self::new(writer))?;
            writer.serialize_event(JsonEvent::EndObject)?;
            Ok(writer)
        }

        fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
            self.writer.serialize_event(JsonEvent::StartArray)?;
            Ok(self)
        }

        fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
            self.writer.serialize_event(JsonEvent::StartArray)?;
            Ok(self)
        }

        fn serialize_tuple_struct(
            self,
            _name: &'static str,
            _len: usize,
        ) -> Result<Self::SerializeTupleStruct, Self::Error> {
            self.writer.serialize_event(JsonEvent::StartArray)?;
            Ok(self)
        }

        fn serialize_tuple_variant(
            self,
            _name: &'static str,
            _variant_index: u32,
            variant: &'static str,
            _len: usize,
        ) -> Result<Self::SerializeTupleVariant, Self::Error> {
            self.writer.serialize_event(JsonEvent::StartObject)?;
            self.writer
                .serialize_event(JsonEvent::ObjectKey(Cow::Borrowed(variant)))?;
            self.writer.serialize_event(JsonEvent::StartArray)?;
            Ok(self)
        }

        fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
            self.writer.serialize_event(JsonEvent::StartObject)?;
            Ok(self)
        }

        fn serialize_struct(
            self,
            _name: &'static str,
            _len: usize,
        ) -> Result<Self::SerializeStruct, Self::Error> {
            self.writer.serialize_event(JsonEvent::StartObject)?;
            Ok(self)
        }

        fn serialize_struct_variant(
            self,
            _name: &'static str,
            _variant_index: u32,
            variant: &'static str,
            _len: usize,
        ) -> Result<Self::SerializeStructVariant, Self::Error> {
            self.writer.serialize_event(JsonEvent::StartObject)?;
            self.writer
                .serialize_event(JsonEvent::ObjectKey(Cow::Borrowed(variant)))?;
            self.writer.serialize_event(JsonEvent::StartObject)?;
            Ok(self)
        }
    }

    macro_rules! imp_ser {
        ($t:ty, $name:ident, $($end:expr),+) => {
            impl<'a, W: Write> $t for JsonSerializer<'a, W> {
                type Ok = &'a mut WriterJsonSerializer<W>;
                type Error = SerDeIoError;

                fn $name<T>(&mut self, value: &T) -> Result<(), Self::Error>
                where
                    T: ?Sized + Serialize,
                {
                    value.serialize(JsonSerializer::new(self.writer))?;
                    Ok(())
                }

                fn end(self) -> Result<Self::Ok, Self::Error> {
                    $(self.writer.serialize_event($end)?;)+
                    Ok(self.writer)
                }
            }
        };
    }

    imp_ser!(SerializeSeq, serialize_element, JsonEvent::EndArray);
    imp_ser!(SerializeTuple, serialize_element, JsonEvent::EndArray);
    imp_ser!(SerializeTupleStruct, serialize_field, JsonEvent::EndArray);
    imp_ser!(
        SerializeTupleVariant,
        serialize_field,
        JsonEvent::EndArray,
        JsonEvent::EndObject
    );

    impl<'a, W: Write> SerializeMap for JsonSerializer<'a, W> {
        type Ok = &'a mut WriterJsonSerializer<W>;
        type Error = SerDeIoError;

        fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize,
        {
            let key = {
                let mut key_buf = Vec::<u8>::new();
                let mut key_json_writer = WriterJsonSerializer::new(&mut key_buf);
                key.serialize(JsonSerializer::new(&mut key_json_writer))?;
                String::from_utf8(key_buf)
                    .map_err(|_| SerDeIoError::custom("can't encode json key to string"))?
            };

            self.writer
                .serialize_event(JsonEvent::ObjectKey(Cow::Owned(key)))?;
            Ok(())
        }

        fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize,
        {
            value.serialize(JsonSerializer::new(self.writer))?;
            Ok(())
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            self.writer.serialize_event(JsonEvent::EndObject)?;
            Ok(self.writer)
        }
    }

    macro_rules! imp_ser_kv {
        ($t:ty, $($end:expr),+) => {
            impl<'a, W: Write> $t for JsonSerializer<'a, W> {
                type Ok = &'a mut WriterJsonSerializer<W>;
                type Error = SerDeIoError;

                fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
                where
                    T: ?Sized + Serialize,
                {
                    self.writer
                        .serialize_event(JsonEvent::ObjectKey(Cow::Borrowed(key)))?;
                    value.serialize(JsonSerializer::new(self.writer))?;
                    Ok(())
                }

                fn end(self) -> Result<Self::Ok, Self::Error> {
                    $(self.writer.serialize_event($end)?;)+
                    Ok(self.writer)
                }
            }
        };
    }

    imp_ser_kv!(SerializeStruct, JsonEvent::EndObject);
    imp_ser_kv!(
        SerializeStructVariant,
        JsonEvent::EndObject,
        JsonEvent::EndObject
    );
}

/// Deserialization: `JsonEventSource` -> `JsonDeserializer` -> `serde::Deserialize`.
mod de {
    use std::{
        borrow::Cow,
        fmt::{Display, Formatter},
        io::Read,
        marker::PhantomData,
        num::IntErrorKind,
        str::FromStr,
    };

    use json_event_parser::{JsonEvent, JsonParseError, ReaderJsonParser};
    use serde::{
        Deserialize, Deserializer,
        de::{
            DeserializeSeed, EnumAccess, MapAccess, SeqAccess, VariantAccess, Visitor,
            value::StrDeserializer,
        },
    };

    use crate::{Skipper, event_name, owned_event, serde::SerDeIoError};

    /// json-event-parser event source, wrapper for `Vec<JsonEvent>`, `&[JsonEvent]`, or `ReaderJsonParser`
    pub trait JsonEventSource {
        /// Returns the next event from the source, or `JsonEvent::Eof` when exhausted.
        fn next_event(&mut self) -> Result<Cow<'_, JsonEvent<'_>>, JsonParseError>;
    }

    impl<R: Read> JsonEventSource for ReaderJsonParser<R> {
        /// Parses the next event from the underlying reader.
        fn next_event(&mut self) -> Result<Cow<'_, JsonEvent<'_>>, JsonParseError> {
            Ok(Cow::Owned(self.parse_next()?))
        }
    }

    impl JsonEventSource for Vec<JsonEvent<'_>> {
        /// Pops the event from the head of the vector (returns `Eof` when empty).
        fn next_event(&mut self) -> Result<Cow<'_, JsonEvent<'_>>, JsonParseError> {
            Ok(Cow::Owned(if self.is_empty() {
                JsonEvent::Eof
            } else {
                self.remove(0)
            }))
        }
    }

    /// JsonEventSource implementation of `&[JsonEvent]`, avoid cost of `Vec<JsonEvent>.remove(0)`
    pub struct JsonEventSlice<'a> {
        pos: usize,
        slice: &'a [JsonEvent<'a>],
    }

    impl<'a> From<&'a [JsonEvent<'a>]> for JsonEventSlice<'a> {
        fn from(value: &'a [JsonEvent<'a>]) -> Self {
            Self::new(value)
        }
    }

    impl<'a> JsonEventSlice<'a> {
        /// Creates an event source over the given event slice, starting at its beginning.
        pub const fn new(slice: &'a [JsonEvent<'a>]) -> Self {
            Self { pos: 0, slice }
        }
    }

    impl<'a> JsonEventSource for JsonEventSlice<'a> {
        /// Returns the next event by advancing an index; events are borrowed, so this avoids allocation.
        fn next_event(&mut self) -> Result<Cow<'_, JsonEvent<'_>>, JsonParseError> {
            if self.pos >= self.slice.len() {
                return Ok(Cow::Owned(JsonEvent::Eof));
            }

            let v = &self.slice[self.pos];
            self.pos += 1;
            Ok(Cow::Borrowed(v))
        }
    }

    /// deserialize a single json value from input json event stream
    /// `JsonEventSource` -> `JsonDeserializer` -> `serde::Deserialize`
    pub struct JsonDeserializer<'a, S: JsonEventSource> {
        source: &'a mut S,
        peek: Option<JsonEvent<'static>>,
    }

    impl<'a, S: JsonEventSource> JsonDeserializer<'a, S> {
        /// Creates a deserializer reading from the given event source.
        pub fn new(source: &'a mut S) -> Self {
            Self { source, peek: None }
        }

        /// Returns a copy of the deserializer with a single peeked (already read) event.
        fn with_peek(self, event: JsonEvent<'static>) -> Self {
            Self {
                source: self.source,
                peek: Some(event),
            }
        }

        /// Like `with_peek`, but a `None` event leaves the deserializer unchanged.
        fn with_opt_peek(self, event: Option<JsonEvent<'static>>) -> Self {
            if let Some(event) = event {
                self.with_peek(event)
            } else {
                self
            }
        }

        /// Returns the next event as owned data, taking the peeked event first if present.
        fn own_next_event(mut self) -> Result<Cow<'a, JsonEvent<'a>>, SerDeIoError> {
            if let Some(event) = self.peek.take() {
                return Ok(Cow::Owned(event));
            }

            self.source.next_event().map_err(Into::into)
        }

        /// Returns the next event, borrowing from the source when possible.
        fn next_event(&mut self) -> Result<Cow<'_, JsonEvent<'_>>, SerDeIoError> {
            if let Some(event) = self.peek.take() {
                return Ok(Cow::Owned(event));
            }

            self.source.next_event().map_err(Into::into)
        }

        /// Returns the next event as an owned `JsonEvent<'static>` (see `crate::owned_event`).
        fn next_event_static(&mut self) -> Result<JsonEvent<'static>, SerDeIoError> {
            if let Some(event) = self.peek.take() {
                return Ok(event);
            }

            self.next_event().map(|v| owned_event(v.as_ref().clone()))
        }

        /// Consume a JSON array of numbers into `buf` as bytes, up to the matching `EndArray`.
        fn consume_byte_buf(&mut self, buf: &mut Vec<u8>) -> Result<(), SerDeIoError> {
            loop {
                match &self.next_event()?.as_ref() {
                    JsonEvent::EndArray => {
                        return Ok(());
                    }
                    JsonEvent::Number(text) => match text.parse::<u8>() {
                        Ok(v) => buf.push(v),
                        Err(err) => {
                            return Err(SerDeIoError::custom(format!(
                                "can't parse \"{}\" as byte while parsing bytes from array: {:#?}",
                                text, err
                            )));
                        }
                    },
                    event => {
                        return Err(SerDeIoError::custom(format!(
                            "unexpect {} while parsing bytes from array",
                            event_name(event)
                        )));
                    }
                }
            }
        }
    }

    /// Macro generating the primitive `deserialize_*` methods:
    /// a `Number` event is parsed to the target type, a `String` event is parsed as a string.
    macro_rules! primitive_visit {
        ($name:ident, $visit_fn:ident, $visit_ty:ty) => {
            fn $name<V>(self, visitor: V) -> Result<V::Value, Self::Error>
            where
                V: Visitor<'de>,
            {
                match &self.own_next_event()?.as_ref() {
                    JsonEvent::String(v) => match v.parse::<$visit_ty>() {
                        Ok(v) => visitor.$visit_fn(v),
                        Err(_) => Err(unexpect_type("String", &visitor)),
                    },
                    JsonEvent::Number(v) => {
                        visitor.$visit_fn(v.parse::<$visit_ty>().map_err(|err| {
                            SerDeIoError::new(std::io::ErrorKind::InvalidInput, err)
                        })?)
                    }
                    event => Err(unexpect_type(event_name(event), &visitor)),
                }
            }
        };
    }

    impl<'a, 'de: 'a, S: JsonEventSource> Deserializer<'de> for JsonDeserializer<'a, S> {
        type Error = SerDeIoError;

        fn deserialize_any<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            match self.next_event_static()? {
                JsonEvent::String(v) => visitor.visit_str(&v),
                JsonEvent::Number(v) => {
                    let num = v.parse::<JsonNumber>()?;
                    num.visit(visitor)
                }
                JsonEvent::Boolean(v) => visitor.visit_bool(v),
                JsonEvent::Null => visitor.visit_none(),
                JsonEvent::StartArray => visitor.visit_seq(self),
                JsonEvent::StartObject => visitor.visit_map(self),
                JsonEvent::Eof => Err(SerDeIoError::eof()),
                event => Err(unexpect_type(event_name(&event), &visitor)),
            }
        }

        primitive_visit!(deserialize_bool, visit_bool, bool);
        primitive_visit!(deserialize_i8, visit_i8, i8);
        primitive_visit!(deserialize_u8, visit_u8, u8);
        primitive_visit!(deserialize_i16, visit_i16, i16);
        primitive_visit!(deserialize_u16, visit_u16, u16);
        primitive_visit!(deserialize_i32, visit_i32, i32);
        primitive_visit!(deserialize_u32, visit_u32, u32);
        primitive_visit!(deserialize_i64, visit_i64, i64);
        primitive_visit!(deserialize_u64, visit_u64, u64);
        primitive_visit!(deserialize_i128, visit_i128, i128);
        primitive_visit!(deserialize_u128, visit_u128, u128);
        primitive_visit!(deserialize_f32, visit_f32, f32);
        primitive_visit!(deserialize_f64, visit_f64, f64);

        fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            match self.own_next_event()?.as_ref() {
                JsonEvent::String(v) => {
                    let mut chars = v.chars();
                    let ch = match chars.next() {
                        Some(v) => v,
                        None => return Err(unexpect_type("String(empty)", &visitor)),
                    };
                    if chars.next().is_some() {
                        return Err(unexpect_type("String(multi chars)", &visitor));
                    }
                    visitor.visit_char(ch)
                }
                event => Err(unexpect_type(event_name(event), &visitor)),
            }
        }

        fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            match self.own_next_event()?.as_ref() {
                JsonEvent::String(v) => visitor.visit_str(v.as_ref()),
                event => Err(unexpect_type(event_name(event), &visitor)),
            }
        }

        fn deserialize_string<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            match self.next_event()?.as_ref() {
                JsonEvent::String(v) => visitor.visit_string(v.to_string()),
                event => Err(unexpect_type(event_name(event), &visitor)),
            }
        }

        fn deserialize_bytes<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            match self.next_event()?.as_ref() {
                JsonEvent::String(v) => visitor.visit_bytes(v.as_bytes()),
                JsonEvent::StartArray => {
                    let mut buf = Vec::<u8>::new();
                    self.consume_byte_buf(&mut buf)?;
                    visitor.visit_bytes(&buf)
                }
                event => Err(unexpect_type(event_name(event), &visitor)),
            }
        }

        fn deserialize_byte_buf<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            let mut buf = Vec::<u8>::new();
            match self.next_event()?.as_ref() {
                JsonEvent::String(v) => buf.extend_from_slice(v.as_bytes()),
                JsonEvent::StartArray => {
                    self.consume_byte_buf(&mut buf)?;
                }
                event => return Err(unexpect_type(event_name(event), &visitor)),
            }
            visitor.visit_byte_buf(buf)
        }

        fn deserialize_option<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            match self.next_event_static()? {
                JsonEvent::Null => visitor.visit_none(),
                event => visitor.visit_some(self.with_peek(event)),
            }
        }

        fn deserialize_unit<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            match &self.next_event()?.as_ref() {
                JsonEvent::Null => visitor.visit_none(),
                event => Err(unexpect_type(event_name(event), &visitor)),
            }
        }

        fn deserialize_unit_struct<V>(
            self,
            _name: &'static str,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            self.deserialize_unit(visitor)
        }

        fn deserialize_newtype_struct<V>(
            self,
            _name: &'static str,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            visitor.visit_newtype_struct(self)
        }

        fn deserialize_seq<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            match self.next_event_static()? {
                JsonEvent::StartArray => visitor.visit_seq(self),
                event => Err(unexpect_type(event_name(&event), &visitor)),
            }
        }

        fn deserialize_tuple<V>(mut self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            match self.next_event_static()? {
                JsonEvent::StartArray => visitor.visit_seq(self),
                event => Err(unexpect_type(event_name(&event), &visitor)),
            }
        }

        fn deserialize_tuple_struct<V>(
            mut self,
            _name: &'static str,
            _len: usize,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            match self.next_event_static()? {
                JsonEvent::StartArray => visitor.visit_seq(self),
                event => Err(unexpect_type(event_name(&event), &visitor)),
            }
        }

        fn deserialize_map<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            match self.next_event()?.as_ref() {
                JsonEvent::StartObject => visitor.visit_map(self),
                event => Err(unexpect_type(event_name(event), &visitor)),
            }
        }

        fn deserialize_struct<V>(
            mut self,
            _name: &'static str,
            _fields: &'static [&'static str],
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            match self.next_event_static()? {
                JsonEvent::StartArray => visitor.visit_seq(self),
                JsonEvent::StartObject => visitor.visit_map(self),
                event => Err(unexpect_type(event_name(&event), &visitor)),
            }
        }

        fn deserialize_enum<V>(
            mut self,
            _name: &'static str,
            _variants: &'static [&'static str],
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            let event = self.next_event_static()?;
            if matches!(event, JsonEvent::String(_) | JsonEvent::Number(_)) {
                visitor.visit_enum(JsonVariantAccess {
                    is_unit: true,
                    peek: Some(event),
                    source: self,
                })
            } else if matches!(event, JsonEvent::StartObject) {
                visitor.visit_enum(JsonVariantAccess {
                    is_unit: false,
                    peek: Some(event),
                    source: self,
                })
            } else {
                Err(unexpect_type(event_name(&event), &visitor))
            }
        }

        fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            self.deserialize_str(visitor)
        }

        fn deserialize_ignored_any<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            let mut skipper = Skipper::new();
            while skipper.skipping() {
                skipper
                    .on_event(self.next_event()?.as_ref())
                    .map_err(SerDeIoError::custom)?;
            }
            visitor.visit_unit()
        }
    }

    struct JsonVariantAccess<'a, S: JsonEventSource> {
        is_unit: bool,
        peek: Option<JsonEvent<'static>>,
        source: JsonDeserializer<'a, S>,
    }

    impl<'a, 'de: 'a, S: JsonEventSource> EnumAccess<'de> for JsonVariantAccess<'a, S> {
        type Error = SerDeIoError;
        type Variant = Self;

        /// Deserializes the variant name using the peeked event, if any.
        fn variant_seed<V>(mut self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
        where
            V: DeserializeSeed<'de>,
        {
            let peek = self.peek.take();
            let val =
                seed.deserialize(JsonDeserializer::new(self.source.source).with_opt_peek(peek))?;
            Ok((val, self))
        }
    }

    impl<'a, 'de: 'a, S: JsonEventSource> VariantAccess<'de> for JsonVariantAccess<'a, S> {
        type Error = SerDeIoError;

        /// Handles a unit variant.
        ///
        /// In the single-field map form (`{"Variant": null}`) the variant value
        /// is deserialized from the event stream.
        fn unit_variant(self) -> Result<(), Self::Error> {
            if self.is_unit {
                return Ok(());
            }
            Deserialize::deserialize(self.source)
        }

        /// Handles a newtype variant by deserializing its payload from the event stream.
        fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
        where
            T: DeserializeSeed<'de>,
        {
            if self.is_unit {
                Err(SerDeIoError::custom(
                    "UnitVariantAccess do **NOT** support newtype_variant_seed",
                ))
            } else {
                seed.deserialize(self.source)
            }
        }

        /// Handles a tuple variant as a JSON array.
        fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            if self.is_unit {
                Err(SerDeIoError::custom(
                    "UnitVariantAccess do **NOT** support tuple_variant",
                ))
            } else {
                Deserializer::deserialize_seq(self.source, visitor)
            }
        }

        /// Handles a struct variant as a JSON object with the given field names.
        fn struct_variant<V>(
            self,
            fields: &'static [&'static str],
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            if self.is_unit {
                Err(SerDeIoError::custom(
                    "UnitVariantAccess do **NOT** support struct_variant",
                ))
            } else {
                Deserializer::deserialize_struct(self.source, "", fields, visitor)
            }
        }
    }

    impl<'a, 'de: 'a, S: JsonEventSource> MapAccess<'de> for JsonDeserializer<'a, S> {
        type Error = SerDeIoError;

        /// Deserializes the next object key from an `ObjectKey` event, returning `None` at `EndObject`.
        fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
        where
            K: DeserializeSeed<'de>,
        {
            let event = self.source.next_event()?;
            match event.as_ref() {
                JsonEvent::ObjectKey(k) => {
                    seed.deserialize(StrDeserializer::new(k.as_ref())).map(Some)
                }
                JsonEvent::EndObject => Ok(None),
                JsonEvent::Eof => Err(SerDeIoError::eof()),
                event => Err(SerDeIoError::custom(format!(
                    "unexpect event for map key: {:#?}",
                    event
                ))),
            }
        }

        /// Deserializes the value following the last key.
        fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
        where
            V: DeserializeSeed<'de>,
        {
            seed.deserialize(JsonDeserializer::new(self.source))
        }
    }

    impl<'a, 'de: 'a, S: JsonEventSource> SeqAccess<'de> for JsonDeserializer<'a, S> {
        type Error = SerDeIoError;

        /// Deserializes the next array element, returning `None` at `EndArray`.
        fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
        where
            T: DeserializeSeed<'de>,
        {
            match self.next_event_static()? {
                JsonEvent::EndArray => Ok(None),
                event => seed
                    .deserialize(JsonDeserializer::new(self.source).with_peek(event))
                    .map(Some),
            }
        }
    }

    /// A parsed JSON number: `i64`, `u64`, or (on integer overflow) `f64`.
    enum JsonNumber {
        I64(i64),
        U64(u64),
        F64(f64),
    }

    impl FromStr for JsonNumber {
        type Err = SerDeIoError;

        /// Parses a number literal, falling back to `f64` on integer overflow.
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let s = s.trim();
            let err = if s.starts_with('-') {
                match s.parse::<i64>() {
                    Ok(v) => return Ok(Self::I64(v)),
                    Err(err) => err,
                }
            } else {
                match s.parse::<u64>() {
                    Ok(v) => return Ok(Self::U64(v)),
                    Err(err) => err,
                }
            };

            if matches!(
                err.kind(),
                IntErrorKind::NegOverflow | IntErrorKind::PosOverflow
            ) {
                match s.parse::<f64>() {
                    Ok(v) => Ok(Self::F64(v)),
                    Err(err) => Err(SerDeIoError::custom(format!("parse number error: {}", err))),
                }
            } else {
                Err(SerDeIoError::custom(format!("parse number error: {}", err)))
            }
        }
    }

    impl JsonNumber {
        /// Visits the visitor with the contained number value.
        fn visit<'de, V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, SerDeIoError> {
            match self {
                JsonNumber::I64(v) => visitor.visit_i64(v),
                JsonNumber::U64(v) => visitor.visit_u64(v),
                JsonNumber::F64(v) => visitor.visit_f64(v),
            }
        }
    }

    /// Builds a `SerDeIoError` describing an unexpected event type, using the visitor's `expecting` for the expected type.
    fn unexpect_type<'de, V: Visitor<'de>>(event_name: &str, visitor: &V) -> SerDeIoError {
        struct ErrMeta<'a, 'de, V: Visitor<'de>> {
            event_name: &'a str,
            visitor: &'a V,
            mark: PhantomData<&'de ()>,
        }

        impl<'a, 'de, V: Visitor<'de>> Display for ErrMeta<'a, 'de, V> {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(f, "UnexpectType[got={}, expect=", self.event_name)?;
                self.visitor.expecting(f)?;
                write!(f, "]")?;
                Ok(())
            }
        }

        SerDeIoError::new(
            std::io::ErrorKind::InvalidInput,
            ErrMeta {
                event_name,
                visitor,
                mark: PhantomData,
            }
            .to_string(),
        )
    }
}
