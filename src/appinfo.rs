use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    release_stage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    atype: Option<String>,
}

impl AppInfo {
    pub fn new(version: Option<&str>, release_stage: Option<&str>, atype: Option<&str>) -> AppInfo {
        AppInfo {
            version: version.map_or_else(|| None, |v| Some(v.to_owned())),
            release_stage: release_stage.map_or_else(|| None, |v| Some(v.to_owned())),
            atype: atype.map_or_else(|| None, |v| Some(v.to_owned())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::AppInfo;
    use serde_json::json;

    #[test]
    fn test_appinfo_to_json() {
        let info = AppInfo::new(Some("1.0.0"), Some("test"), Some("rust"));

        assert_eq!(
            serde_json::to_value(&info).unwrap(),
            json!({
                "version": "1.0.0",
                "releaseStage": "test",
                "type": "rust"
            })
        );
    }

    #[test]
    fn test_appinfo_with_version_to_json() {
        let info = AppInfo::new(Some("1.0.0"), None, None);

        assert_eq!(
            serde_json::to_value(&info).unwrap(),
            json!({
                "version": "1.0.0"
            })
        );
    }

    #[test]
    fn test_appinfo_with_release_stage_to_json() {
        let info = AppInfo::new(None, Some("test"), None);

        assert_eq!(
            serde_json::to_value(&info).unwrap(),
            json!({
                "releaseStage": "test"
            })
        );
    }

    #[test]
    fn test_appinfo_with_type_to_json() {
        let info = AppInfo::new(None, None, Some("rust"));

        assert_eq!(
            serde_json::to_value(&info).unwrap(),
            json!({
                "type": "rust"
            })
        );
    }
}
