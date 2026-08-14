# json-event-parser-blocks

[`json-event-parser`] 的功能扩展 crate，为 JSON 事件流提供高级抽象：

- **`Skipper`** — 从事件流中跳过整个嵌套对象或数组
- **`serde` 支持**（可选特性）— 通过 `serde` 生态系统序列化/反序列化 Rust 类型

[![Latest Version](https://img.shields.io/crates/v/json-event-parser-blocks.svg)](https://crates.io/crates/json-event-parser-blocks)
[![Released API docs](https://docs.rs/json-event-parser-blocks/badge.svg)](https://docs.rs/json-event-parser-blocks)

## 安装

将 crate 添加到 `Cargo.toml`：

```toml
[dependencies]
json-event-parser-blocks = "0.1"

# 可选：启用 serde 支持
[dependencies.json-event-parser-blocks]
version = "0.1"
features = ["serde"]
```

## `Skipper`

`Skipper` 允许你从事件流中跳过任意嵌套的 JSON 值（对象、数组、基本类型），而无需对其进行反序列化。它内部跟踪嵌套深度，并在目标值完全消费后自动停止。

### 示例：跳过值

```rust
fn skip_a_value(reader: &mut ReaderJsonParser) {
    let mut skipper = Skipper::new();
    while skipper.skipping() {
        let event = reader.parse_next().unwrap();
        skipper.on_event(&event).unwrap();
    }
}
```

### 示例：跳过时捕获事件

将 `Skipper` 与事件缓存结合使用，以便检查被跳过的内容：

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

### API 概览

| 方法 | 描述 |
|------|------|
| `Skipper::new()` | 创建新的 skipper 实例 |
| `skipping()` | 跳过是否仍在进行中 |
| `on_event(&event)` | 传入下一个事件，返回是否继续跳过 |
| `reset()` | 重置 skipper 以便复用 |

## `serde` 支持

启用 `serde` 特性后，crate 提供了完整的 [`serde`] Serializer 和 Deserializer，可与 `json-event-parser` 的事件模型配合使用。

### 序列化（`serde::Serialize`）

将实现 `Serialize` 的类型转换为 JSON 事件流：

```rust
use serde::Serialize;

fn encode<T: Serialize>(value: &T, writer: &mut WriterJsonSerializer) {
    value
        .serialize(JsonSerializer::new(writer))
        .unwrap();
}
```

### 反序列化（`serde::Deserialize`）

从 JSON 事件流反序列化为 Rust 类型：

```rust
use serde::Deserialize;

fn decode<'de, T: Deserialize<'de>>(reader: &mut ReaderJsonParser) -> T {
    T::deserialize(JsonDeserializer::new(reader)).unwrap()
}
```

### 自定义事件源

`JsonDeserializer` 可与任何实现了 `JsonEventSource` 的类型配合使用：

| 事件源 | 描述 |
|--------|------|
| `ReaderJsonParser<R>` | 直接从 I/O reader 流式读取 |
| `Vec<JsonEvent<'_>>` | 从拥有的事件缓冲区反序列化 |
| `JsonEventSlice<'_>` | 从借用的事件切片反序列化（零拷贝） |

## 错误处理

所有 serde 操作均返回 `SerDeIoError`，其封装了以下错误类型：

- `std::io::Error` — I/O 操作失败
- `JsonParseError` — 格式错误的事件
- `Box<dyn Error + Send + Sync>` — 自定义错误

## 许可证

本项目采用 [MIT 许可证](LICENSE) 授权。

[`json-event-parser`]: https://crates.io/crates/json-event-parser
[`serde`]: https://crates.io/crates/serde
