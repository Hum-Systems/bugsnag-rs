use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceInfo {
    os_version: String,
    hostname: String,
}

impl DeviceInfo {
    pub fn new(version: &str, name: &str) -> DeviceInfo {
        DeviceInfo {
            os_version: version.to_owned(),
            hostname: name.to_owned(),
        }
    }

    pub fn generate() -> DeviceInfo {
        let mut version = sys_info::os_type().unwrap_or("Unknown".to_owned());
        version.push(':');
        version.push_str(&sys_info::os_release().unwrap_or("u.k.n.o.w.n".to_owned()));

        let hostname = sys_info::hostname().unwrap_or("UnknownHost".to_owned());

        DeviceInfo::new(version.as_str(), hostname.as_str())
    }

    pub fn set_os_version(&mut self, version: &str) {
        self.os_version = version.to_owned();
    }

    pub fn set_hostname(&mut self, name: &str) {
        self.hostname = name.to_owned();
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::DeviceInfo;

    #[test]
    fn test_deviceinfo_to_json() {
        let info = DeviceInfo::new("1.0.0", "testmachine");

        assert_eq!(
            serde_json::to_value(&info).unwrap(),
            json!({
                "osVersion": "1.0.0",
                "hostname": "testmachine"
            })
        );
    }

    #[test]
    fn test_deviceinfo_to_json_with_set() {
        let mut info = DeviceInfo::generate();
        info.set_hostname("testmachine3");
        info.set_os_version("3.0.0");

        assert_eq!(
            serde_json::to_value(&info).unwrap(),
            json!({
                "osVersion": "3.0.0",
                "hostname": "testmachine3"
            })
        );
    }
}
