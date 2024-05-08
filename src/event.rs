use super::appinfo::AppInfo;
use super::deviceinfo::DeviceInfo;
use super::exception::Exception;
use super::user::User;
use super::Severity;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Event<'a> {
    exceptions: &'a [Exception<'a>],
    #[serde(skip_serializing_if = "Option::is_none")]
    severity: Option<&'a Severity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    context: Option<&'a str>,
    device: &'a DeviceInfo,
    #[serde(skip_serializing_if = "Option::is_none")]
    app: &'a Option<AppInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user: &'a Option<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    meta_data: &'a Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    grouping_hash: Option<&'a str>,
}

impl<'a> Event<'a> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        exceptions: &'a [Exception],
        severity: Option<&'a Severity>,
        context: Option<&'a str>,
        grouping_hash: Option<&'a str>,
        device: &'a DeviceInfo,
        app: &'a Option<AppInfo>,
        user: &'a Option<User>,
        meta_data: &'a Option<serde_json::Value>,
    ) -> Event<'a> {
        Event {
            exceptions,
            severity,
            context,
            device,
            app,
            user,
            meta_data,
            grouping_hash,
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::{AppInfo, DeviceInfo, Event, Severity};
    use crate::user::User;

    #[test]
    fn test_event_to_json() {
        let empty_vec = Vec::new();
        let device = DeviceInfo::new("1.0.0", "testmachine");
        let app = None;
        let user = None;
        let metadata = None;
        let evt = Event::new(
            &empty_vec,
            Some(&Severity::Error),
            None,
            None,
            &device,
            &app,
            &user,
            &metadata,
        );

        assert_eq!(
            serde_json::to_value(&evt).unwrap(),
            json!({
                "exceptions": [],
                "severity": "error",
                "device": {
                    "osVersion": "1.0.0",
                    "hostname": "testmachine"
                }
            })
        );
    }

    #[test]
    fn test_event_with_context_to_json() {
        let empty_vec = Vec::new();
        let device = DeviceInfo::new("1.0.0", "testmachine");
        let app = None;
        let user = None;
        let metadata = None;
        let evt = Event::new(
            &empty_vec,
            Some(&Severity::Error),
            Some("test/context"),
            None,
            &device,
            &app,
            &user,
            &metadata,
        );

        assert_eq!(
            serde_json::to_value(&evt).unwrap(),
            json!({
                "exceptions": [],
                "severity": "error",
                "context": "test/context",
                "device": {
                    "osVersion": "1.0.0",
                    "hostname": "testmachine"
                }
            })
        );
    }

    #[test]
    fn test_event_with_app_info_to_json() {
        let empty_vec = Vec::new();
        let device = DeviceInfo::new("1.0.0", "testmachine");
        let app = Some(AppInfo::new(Some("1.0.0"), Some("test"), Some("rust")));
        let user = None;
        let metadata = None;
        let evt = Event::new(
            &empty_vec,
            Some(&Severity::Error),
            None,
            None,
            &device,
            &app,
            &user,
            &metadata,
        );

        assert_eq!(
            serde_json::to_value(&evt).unwrap(),
            json!({
                "exceptions": [],
                "severity": "error",
                "device": {
                    "osVersion": "1.0.0",
                    "hostname": "testmachine"
                },
                "app": {
                    "version": "1.0.0",
                    "releaseStage": "test",
                    "type": "rust"
                }
            })
        );
    }

    #[test]
    fn test_event_with_user_to_json() {
        let empty_vec = Vec::new();
        let device = DeviceInfo::new("1.0.0", "testmachine");
        let app = None;
        let user = Some(User::new("123456789", "testuser", "test@user.com"));
        let metadata = None;
        let evt = Event::new(
            &empty_vec,
            Some(&Severity::Error),
            None,
            None,
            &device,
            &app,
            &user,
            &metadata,
        );

        assert_eq!(
            serde_json::to_value(&evt).unwrap(),
            json!({
                "exceptions": [],
                "severity": "error",
                "device": {
                    "osVersion": "1.0.0",
                    "hostname": "testmachine"
                },
                "user": {
                    "id": "123456789",
                    "name": "testuser",
                    "email": "test@user.com"
                }
            })
        );
    }

    #[test]
    fn test_event_with_metadata_to_json() {
        let empty_vec = Vec::new();
        let device = DeviceInfo::new("1.0.0", "testmachine");
        let app = None;
        let user = None;

        let metadata = Some(json!({
            "test": "DATA_META_TEST",
            "meta": "test meta data",
            "data": {
                "boolean": false,
                "float": 1.0,
                "number": 42
            }
        }));

        let evt = Event::new(
            &empty_vec,
            Some(&Severity::Error),
            None,
            None,
            &device,
            &app,
            &user,
            &metadata,
        );

        assert_eq!(
            serde_json::to_value(&evt).unwrap(),
            json!({
                "exceptions": [],
                "severity": "error",
                "device": {
                    "osVersion": "1.0.0",
                    "hostname": "testmachine"
                },
                "metaData": {
                    "test": "DATA_META_TEST",
                    "meta": "test meta data",
                    "data": {
                        "boolean": false,
                        "float": 1.0,
                        "number": 42
                    }
                }
            })
        );
    }
}
