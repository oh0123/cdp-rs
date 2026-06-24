use super::cdp_protocol::{CdpEvent, CdpEventMessage};
use crate::error::Result;
use cdp_protocol::types::Event;

/// Parses a raw CDP event message into a strongly typed [`CdpEvent`].
pub(crate) fn parse_cdp_event(event_msg: &CdpEventMessage) -> Result<CdpEvent> {
    match parse_generated_event(event_msg) {
        Ok(event) => Ok(event),
        Err(err) => {
            tracing::debug!(
                "Falling back to unknown CDP event for {}: {:?}",
                event_msg.method,
                err
            );
            Ok(CdpEvent::UnhnownEvent(event_msg.clone()))
        }
    }
}

fn parse_generated_event(event_msg: &CdpEventMessage) -> Result<CdpEvent> {
    let raw_event = serde_json::json!({
        "method": event_msg.method.clone(),
        "params": event_msg.params.clone(),
    });
    let event: Event = serde_json::from_value(raw_event)?;
    tracing::debug!("Parsed {} event", event_msg.method);
    Ok(CdpEvent::Event(Box::new(event)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use cdp_protocol::{page, runtime, target, types::Event};
    use serde_json::json;

    #[test]
    fn parses_inspector_target_crashed_event() {
        let event_msg = CdpEventMessage {
            method: "Inspector.targetCrashed".to_string(),
            params: serde_json::Value::Null,
            session_id: Some("session-1".to_string()),
        };

        let event = parse_cdp_event(&event_msg).expect("event should parse");
        assert!(matches!(
            event,
            CdpEvent::Event(boxed)
                if matches!(*boxed, Event::InspectorTargetCrashed(_))
        ));
    }

    #[test]
    fn parses_target_crashed_event() {
        let event_msg = CdpEventMessage {
            method: "Target.targetCrashed".to_string(),
            params: json!({
                "targetId": "target-1",
                "status": "crashed",
                "errorCode": 42
            }),
            session_id: None,
        };

        let event = parse_cdp_event(&event_msg).expect("event should parse");
        let crashed =
            target::events::TargetCrashedEvent::try_from(event).expect("event should convert");

        assert_eq!(crashed.params.target_id, "target-1");
        assert_eq!(crashed.params.status, "crashed");
        assert_eq!(crashed.params.error_code, 42);
    }

    #[test]
    fn parses_generated_page_file_chooser_event() {
        let event_msg = CdpEventMessage {
            method: "Page.fileChooserOpened".to_string(),
            params: json!({
                "frameId": "frame-1",
                "mode": "selectSingle"
            }),
            session_id: Some("session-1".to_string()),
        };

        let event = parse_cdp_event(&event_msg).expect("event should parse");
        let file_chooser =
            page::events::FileChooserOpenedEvent::try_from(event).expect("event should convert");

        assert_eq!(file_chooser.params.frame_id, "frame-1");
    }

    #[test]
    fn parses_generated_runtime_exception_event() {
        let event_msg = CdpEventMessage {
            method: "Runtime.exceptionThrown".to_string(),
            params: json!({
                "timestamp": 1.0,
                "exceptionDetails": {
                    "exceptionId": 1,
                    "text": "boom",
                    "lineNumber": 1,
                    "columnNumber": 2
                }
            }),
            session_id: Some("session-1".to_string()),
        };

        let event = parse_cdp_event(&event_msg).expect("event should parse");
        let exception =
            runtime::events::ExceptionThrownEvent::try_from(event).expect("event should convert");

        assert_eq!(exception.params.exception_details.text, "boom");
    }

    #[test]
    fn falls_back_to_unknown_event() {
        let event_msg = CdpEventMessage {
            method: "Unknown.event".to_string(),
            params: serde_json::Value::Null,
            session_id: None,
        };

        let event = parse_cdp_event(&event_msg).expect("unknown event should not fail");

        assert!(matches!(event, CdpEvent::UnhnownEvent(_)));
    }
}
