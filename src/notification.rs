use super::event::Event;
use serde::Serialize;

const NOTIFIER_NAME: &str = "Bugsnag Rust";
const NOTIFIER_VERSION: &str = env!("CARGO_PKG_VERSION");
const NOTIFIER_URL: &str = "https://github.com/Hum-Systems/bugsnag-rs";
pub const PAYLOAD_VERSION: &str = "5";

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct Notifier {
    name: &'static str,
    version: &'static str,
    url: &'static str,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Notification<'a> {
    payload_version: &'static str,
    notifier: Notifier,
    events: &'a [Event<'a>],
}

impl<'a> Notification<'a> {
    pub fn new(events: &'a [Event]) -> Notification<'a> {
        Notification {
            payload_version: PAYLOAD_VERSION,
            notifier: Notifier {
                name: NOTIFIER_NAME,
                version: NOTIFIER_VERSION,
                url: NOTIFIER_URL,
            },
            events,
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::super::{deviceinfo, event, exception, stacktrace};
    use super::{Notification, NOTIFIER_NAME, NOTIFIER_URL, NOTIFIER_VERSION, PAYLOAD_VERSION};

    #[test]
    fn test_notification_to_json() {
        let empty_vec = Vec::new();
        let notification = Notification::new(&empty_vec);

        assert_eq!(
            serde_json::to_value(&notification).unwrap(),
            json!({
                "payloadVersion": PAYLOAD_VERSION,
                "notifier": {
                    "name": NOTIFIER_NAME,
                    "version": NOTIFIER_VERSION,
                    "url": NOTIFIER_URL,
                },
                "events": []
            })
        );
    }

    #[test]
    fn test_notification_with_event_to_json() {
        let frames = vec![stacktrace::Frame::new("test.rs", 400, "test", false)];
        let exceptions = vec![exception::Exception::new("Assert", "Assert", &frames)];
        let device = deviceinfo::DeviceInfo::new("1.0.0", "testmachine");
        let app = None;
        let user = None;
        let metadata = None;
        let events = vec![event::Event::new(
            &exceptions,
            None,
            None,
            None,
            &device,
            &app,
            &user,
            &metadata,
        )];

        let notification = Notification::new(&events);

        assert_eq!(
            serde_json::to_value(&notification).unwrap(),
            json!({
                "payloadVersion": PAYLOAD_VERSION,
                "notifier": {
                    "name": NOTIFIER_NAME,
                    "version": NOTIFIER_VERSION,
                    "url": NOTIFIER_URL,
                },
                "events": [
                    {
                        "exceptions": [
                            {
                                "errorClass": "Assert",
                                "message": "Assert",
                                "stacktrace": [
                                    {
                                        "file": "test.rs",
                                        "lineNumber": 400,
                                        "method": "test",
                                        "inProject": false
                                    }
                                ]
                            }
                        ],
                        "device": {
                            "osVersion": "1.0.0",
                            "hostname": "testmachine"
                        }
                    }
                ]
            })
        );
    }
}
