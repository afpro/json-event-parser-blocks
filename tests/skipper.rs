use json_event_parser::{JsonEvent, ReaderJsonParser};
use json_event_parser_blocks::Skipper;

fn skip_test(json: &str, check_y: &str) {
    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    enum State {
        WaitScope,
        InScopeWaitX,
        InScopeSkippingWaitY,
        InScopeWaitYValue,
        WaitEndScope,
        Done,
    }

    let mut reader = ReaderJsonParser::new(json.as_bytes());
    let mut skipper = Skipper::new();
    let mut state = State::WaitScope;
    let mut y: Option<String> = None;

    loop {
        let event = reader.parse_next().unwrap();
        match event {
            JsonEvent::StartObject if state == State::WaitScope => {
                state = State::InScopeWaitX;
            }
            JsonEvent::EndObject if state == State::WaitEndScope => {
                state = State::Done;
            }
            JsonEvent::ObjectKey(k) if state == State::InScopeWaitX && k == "x" => {
                state = State::InScopeSkippingWaitY;
            }
            event if state == State::InScopeSkippingWaitY && skipper.skipping() => {
                skipper.on_event(&event).unwrap();
            }
            JsonEvent::ObjectKey(k) if state == State::InScopeSkippingWaitY && k == "y" => {
                state = State::InScopeWaitYValue;
            }
            JsonEvent::Number(v) if state == State::InScopeWaitYValue => {
                state = State::WaitEndScope;
                y = Some(v.to_string());
            }
            JsonEvent::Eof => {
                break;
            }
            event => {
                panic!("unexpected event {:?} of state {:?}", event, state);
            }
        }
    }

    assert!(!skipper.skipping());
    assert_eq!(state, State::Done);
    assert_eq!(y.as_deref(), Some(check_y));
}

#[test]
fn skip_single_value() {
    skip_test(
        r#"
                {
                    "x": 1,
                    "y": 2
                }
            "#,
        "2",
    );
}

#[test]
fn skip_object() {
    skip_test(
        r#"
                {
                    "x": {"sub_a": true, "sub_b": "text", "sub_c": []},
                    "y": 2
                }
            "#,
        "2",
    );
}

#[test]
fn skip_array() {
    skip_test(
        r#"
                {
                    "x": [1, true, "text", {}],
                    "y": 2
                }
            "#,
        "2",
    );
}
