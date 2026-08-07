# 这是一个 `json-event-parser` 的功能扩展

1. 增加了`Skipper`类型,用于从event stream中跳过一整个object或者array
2. 增加了对`serde`库的支持,可以方便的将event stream与`serde`类型互相转化

## `Skipper`的使用

`Skipper`的使用方法demo(**注意这里仅演示用法,没有做错误处理**)

```rust
fn skip_a_value(reader: &mut ReaderJsonParser) {
    let mut skipper = Skipper::new();
    while skipper.skipping() {
        let event = reader.parse_next().unwrap();
        skipper.on_event(&event).unwrap();
    }
}
```

### 与`serde`结合处理数据片段
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

## `serde`支持

### Serialize (**注意这里仅演示用法,没有做错误处理**)

```rust
fn encode<T: Serialize>(value: &T, writer: &mut WriterJsonSerializer) {
    value
        .serialize(JsonSerializer::new(writer))
        .unwrap();
}
```

### Deserialize (**注意这里仅演示用法,没有做错误处理**)

```rust
fn decode<'de, T: Deserialize<'de>>(reader: &mut ReaderJsonParser) -> ValueObject {
    T::deserialize(JsonDeserializer::new(reader)).unwrap()
}
```
