# json-event-parser-blocks

An extension crate for [`json-event-parser`] that provides high-level abstractions for working with JSON event streams:

- **`Skipper`** — skip entire nested objects or arrays from an event stream in a single call
- **`serde` support** (optional feature) — serialize/deserialize Rust types via the `serde` ecosystem

[![Latest Version](https://img.shields.io/crates/v/json-event-parser-blocks.svg)](https://crates.io/crates/json-event-parser-blocks)
[![Released API docs](https://docs.rs/json-event-parser-blocks/badge.svg)](https://docs.rs/json-event-parser-blocks)

## Installation

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
json-event-parser-blocks = "0.1"

# Optional: enable serde support
[dependencies.json-event-parser-blocks]
version = "0.1"
features = ["serde"]
```

## `Skipper`

`Skipper` lets you skip over arbitrary nested JSON values (objects, arrays, primitives) from an event stream without deserializing them. It tracks nesting depth internally and automatically stops when the target value is fully consumed.

### Example: Skip a Value

```rust
fn skip_a_value(reader: &mut ReaderJsonParser) {
    let mut skipper = Skipper::new();
    while skipper.skipping() {
        let event = reader.parse_next().unwrap();
        skipper.on_event(&event).unwrap();
    }
}
```

### Example: Capture Events While Skipping

Use `Skipper` alongside event buffering when you need to inspect what was skipped:

```rust
fn parse_a_value<'de, T: Deserialize<'de>>(reader: &mut ReaderJsonParser) -> T
where
    T: Deserialize<'de>,
{
    let mut skipper = Skipper::new();
    let mut events = Vec::<JsonEvent<'static>>::new();

    while skipper.skipping() {
        let event = reader.parse_next().unwrap();
        skipper.on_event(&event).unwrap();
        events.push(owned_event(event));
    }

    T::deserialize(JsonDeserializer::new(events)).unwrap()
}
```

### API Overview

| Method | Description |
|--------|-------------|
| `Skipper::new()` | Create a new skipper instance |
| `skipping()` | Returns `true` if skipping is still in progress |
| `on_event(&event)` | Feed the next event; returns whether skipping continues |
| `reset()` | Reset the skipper for reuse |

## `serde` Support

When the `serde` feature is enabled, the crate provides a full [`serde`] Serializer and Deserializer that work with `json-event-parser`'s event-based model.

### Serialization (`serde::Serialize`)

Convert any type that implements `Serialize` into a JSON event stream:

```rust
use serde::Serialize;

fn encode<T: Serialize>(value: &T, writer: &mut WriterJsonSerializer) {
    value
        .serialize(JsonSerializer::new(writer))
        .unwrap();
}
```

### Deserialization (`serde::Deserialize`)

Deserialize a JSON event stream into a Rust type:

```rust
use serde::Deserialize;

fn decode<'de, T: Deserialize<'de>>(reader: &mut ReaderJsonParser) -> T {
    T::deserialize(JsonDeserializer::new(reader)).unwrap()
}
```

### Custom Event Sources

`JsonDeserializer` works with any type implementing `JsonEventSource`:

| Source | Description |
|--------|-------------|
| `ReaderJsonParser<R>` | Stream directly from an I/O reader |
| `Vec<JsonEvent<'_>>` | Deserialize from a owned event buffer |
| `JsonEventSlice<'_>` | Deserialize from a borrowed event slice (zero-copy) |

## Error Handling

All serde operations return `SerDeIoError`, which wraps:

- `std::io::Error` — I/O failures
- `JsonParseError` — malformed JSON events
- `Box<dyn Error + Send + Sync>` — custom errors

## License

This project is licensed under the [MIT License](LICENSE).

[`json-event-parser`]: https://crates.io/crates/json-event-parser
[`serde`]: https://crates.io/crates/serde
