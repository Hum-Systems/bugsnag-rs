use super::appinfo::AppInfo;
use super::deviceinfo::DeviceInfo;
use super::exception::Exception;
use super::user::User;
use super::Severity;

pub const PAYLOAD_VERSION: u32 = 4;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Event<'a> {
    payload_version: u32,
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
            payload_version: PAYLOAD_VERSION,
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
    use crate::user::User;

    use super::{AppInfo, DeviceInfo, Event, Severity, PAYLOAD_VERSION};
    use serde_test::{assert_ser_tokens, Token};

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

        assert_ser_tokens(
            &evt,
            &[
                Token::Struct {
                    name: "Event",
                    len: 4,
                },
                Token::Str("payloadVersion"),
                Token::U32(PAYLOAD_VERSION),
                Token::Str("exceptions"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("severity"),
                Token::Some,
                Token::UnitVariant {
                    name: "Severity",
                    variant: "error",
                },
                Token::Str("device"),
                Token::Struct {
                    name: "DeviceInfo",
                    len: 2,
                },
                Token::Str("osVersion"),
                Token::Str("1.0.0"),
                Token::Str("hostname"),
                Token::Str("testmachine"),
                Token::StructEnd,
                Token::StructEnd,
            ],
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

        assert_ser_tokens(
            &evt,
            &[
                Token::Struct {
                    name: "Event",
                    len: 5,
                },
                Token::Str("payloadVersion"),
                Token::U32(PAYLOAD_VERSION),
                Token::Str("exceptions"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("severity"),
                Token::Some,
                Token::UnitVariant {
                    name: "Severity",
                    variant: "error",
                },
                Token::Str("context"),
                Token::Some,
                Token::Str("test/context"),
                Token::Str("device"),
                Token::Struct {
                    name: "DeviceInfo",
                    len: 2,
                },
                Token::Str("osVersion"),
                Token::Str("1.0.0"),
                Token::Str("hostname"),
                Token::Str("testmachine"),
                Token::StructEnd,
                Token::StructEnd,
            ],
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

        assert_ser_tokens(
            &evt,
            &[
                Token::Struct {
                    name: "Event",
                    len: 5,
                },
                Token::Str("payloadVersion"),
                Token::U32(PAYLOAD_VERSION),
                Token::Str("exceptions"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("severity"),
                Token::Some,
                Token::UnitVariant {
                    name: "Severity",
                    variant: "error",
                },
                Token::Str("device"),
                Token::Struct {
                    name: "DeviceInfo",
                    len: 2,
                },
                Token::Str("osVersion"),
                Token::Str("1.0.0"),
                Token::Str("hostname"),
                Token::Str("testmachine"),
                Token::StructEnd,
                Token::Str("app"),
                Token::Some,
                Token::Struct {
                    name: "AppInfo",
                    len: 3,
                },
                Token::Str("version"),
                Token::Some,
                Token::Str("1.0.0"),
                Token::Str("releaseStage"),
                Token::Some,
                Token::Str("test"),
                Token::Str("type"),
                Token::Some,
                Token::Str("rust"),
                Token::StructEnd,
                Token::StructEnd,
            ],
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

        assert_ser_tokens(
            &evt,
            &[
                Token::Struct {
                    name: "Event",
                    len: 5,
                },
                Token::Str("payloadVersion"),
                Token::U32(PAYLOAD_VERSION),
                Token::Str("exceptions"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("severity"),
                Token::Some,
                Token::UnitVariant {
                    name: "Severity",
                    variant: "error",
                },
                Token::Str("device"),
                Token::Struct {
                    name: "DeviceInfo",
                    len: 2,
                },
                Token::Str("osVersion"),
                Token::Str("1.0.0"),
                Token::Str("hostname"),
                Token::Str("testmachine"),
                Token::StructEnd,
                Token::Str("user"),
                Token::Some,
                Token::Struct {
                    name: "User",
                    len: 3,
                },
                Token::Str("id"),
                Token::Some,
                Token::Str("123456789"),
                Token::Str("name"),
                Token::Some,
                Token::Str("testuser"),
                Token::Str("email"),
                Token::Some,
                Token::Str("test@user.com"),
                Token::StructEnd,
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn test_event_with_metadata_to_json() {
        let empty_vec = Vec::new();
        let device = DeviceInfo::new("1.0.0", "testmachine");
        let app = None;
        let user = None;

        #[derive(Debug, Serialize)]
        struct TestMetaData {
            data: NestedTestMetaData,
            meta: String,
            test: String,
        }
        #[derive(Debug, Serialize)]
        struct NestedTestMetaData {
            boolean: bool,
            float: f64,
            number: u64,
        }
        let test_metadata = TestMetaData {
            data: NestedTestMetaData {
                boolean: false,
                float: 1.0,
                number: 42,
            },
            meta: "test meta data".to_string(),
            test: "DATA_META_TEST".to_string(),
        };

        let metadata = Some(serde_json::to_value(test_metadata).unwrap());

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

        assert_ser_tokens(
            &evt,
            &[
                Token::Struct {
                    name: "Event",
                    len: 5,
                },
                Token::Str("payloadVersion"),
                Token::U32(PAYLOAD_VERSION),
                Token::Str("exceptions"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("severity"),
                Token::Some,
                Token::UnitVariant {
                    name: "Severity",
                    variant: "error",
                },
                Token::Str("device"),
                Token::Struct {
                    name: "DeviceInfo",
                    len: 2,
                },
                Token::Str("osVersion"),
                Token::Str("1.0.0"),
                Token::Str("hostname"),
                Token::Str("testmachine"),
                Token::StructEnd,
                Token::Str("metaData"),
                Token::Some,
                Token::Map { len: Some(3) },
                Token::Str("data"),
                Token::Map { len: Some(3) },
                Token::Str("boolean"),
                Token::Bool(false),
                Token::Str("float"),
                Token::F64(1.0),
                Token::Str("number"),
                Token::U64(42),
                Token::MapEnd,
                Token::Str("meta"),
                Token::Str("test meta data"),
                Token::Str("test"),
                Token::Str("DATA_META_TEST"),
                Token::MapEnd,
                Token::StructEnd,
            ],
        );
    }
}
