# Extension for `json-event-parser`

1. Added the `Skipper` type, which allows skipping entire objects or arrays from an event stream.
2. Added support for the `serde` crate, enabling convenient conversion between event streams and `serde`-compatible types.

## Using `Skipper`

The following demonstrates how to use `Skipper` (**Note: This is for demonstration purposes only and does not include error handling.**):

```rust
fn skip_a_value(reader: &mut ReaderJsonParser) {
    let mut skipper = Skipper::new();
    while skipper.skipping() {
        let event = reader.parse_next().unwrap();
        skipper.on_event(&event).unwrap();
    }
}
```

### Processing Data Segments with `serde`
```rust
fn parse_a_value<'de, T: Deserialize<'de>>(reader: &mut ReaderJsonParser) -> T {
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

## `serde` Support

### Serialize (Note: This is for demonstration purposes only and does not include error handling.)

```rust
fn encode<T: Serialize>(value: &T, writer: &mut WriterJsonSerializer) {
    value
        .serialize(JsonSerializer::new(writer))
        .unwrap();
}
```

### Deserialize (Note: This is for demonstration purposes only and does not include error handling.)

```rust
fn decode<'de, T: Deserialize<'de>>(reader: &mut ReaderJsonParser) -> ValueObject {
    T::deserialize(JsonDeserializer::new(reader)).unwrap()
}
```
