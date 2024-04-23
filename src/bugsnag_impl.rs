use serde::Serialize;
use serde_json::json;

use super::{appinfo, deviceinfo, event, exception, notification, stacktrace, user};

use std::error::Error as StdError;
use std::fmt;
use std::fs::DirEntry;

const NOTIFY_URL: &str = "https://notify.bugsnag.com";
const OFFLINE_REPORT_PREFIX: &str = "bugsnag_report";

#[derive(Debug, PartialEq)]
pub enum Error {
    /// The conversion to json failed.
    JsonConversionFailed,
    /// While transferring the json to Bugsnag, a problem occurred.
    /// This error does not reflect if Bugsnag rejected the json.
    JsonTransferFailed,
    /// Transfer failed and subsequent attempt to store json to offline_storage failed as well.
    JsonTransferAndStorageFailed,
    /// No storage has been specified or could not be read
    OfflineStorageError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl StdError for Error {
    fn description(&self) -> &'static str {
        match *self {
            Error::JsonConversionFailed => "conversion to json failed",
            Error::JsonTransferFailed => {
                "while transferring the json to Bugsnag, a problem occurred"
            },
            Error::JsonTransferAndStorageFailed => {
                "transferring json to Bugsnag failed and subsequent attempt to store json for retransmission failed as well"
            },
            Error::OfflineStorageError => {
                "reading from / writing to offline storage failed"
            }
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    Error,
    Warning,
    Info,
}

#[derive(Debug, Clone)]
pub struct Bugsnag {
    api_key: String,
    device_info: deviceinfo::DeviceInfo,
    app_info: Option<appinfo::AppInfo>,
    user: Option<user::User>,
    project_source_dir: String,
    offline_storage: Option<String>,
}

/// Builder for creating the notification that will be send to Bugsnag.
/// If the object is dropped, the notification is send to Bugsnag.
pub struct NotifyBuilder<'a, 'bugsnag> {
    bugsnag: &'bugsnag Bugsnag,
    error_class: &'a str,
    message: &'a str,
    send_executed: bool,
    methods_to_ignore: Option<&'a [&'a str]>,
    context: Option<&'a str>,
    metadata: Option<serde_json::Value>,
    severity: Option<Severity>,
    grouping_hash: Option<&'a str>,
}

impl<'a, 'bugsnag> NotifyBuilder<'a, 'bugsnag> {
    fn new(
        bugsnag: &'bugsnag Bugsnag,
        error_class: &'a str,
        message: &'a str,
    ) -> NotifyBuilder<'a, 'bugsnag> {
        NotifyBuilder {
            bugsnag,
            error_class,
            message,
            send_executed: false,
            methods_to_ignore: None,
            context: None,
            metadata: None,
            severity: None,
            grouping_hash: None,
        }
    }

    /// Sets a list of methods that should be marked as not belonging
    /// to the project when the stacktrace is generated. The Bugsnag web
    /// interface will use this information to hide unnecessary data.
    /// To check if a method should be marked as not belonging to the
    /// project, the method name reported by the stacktrace is checked if it
    /// contains a method name in this list.
    pub fn methods_to_ignore(mut self, val: &'a [&'a str]) -> Self {
        self.methods_to_ignore = Some(val);
        self
    }

    /// Sets a context that describes the state of the application while the error occurred.
    pub fn context(mut self, val: &'a str) -> Self {
        self.context = Some(val);
        self
    }

    ///
    pub fn metadata(mut self, val: &impl Serialize) -> Result<Self, Error> {
        let json_val = match serde_json::to_value(val) {
            Ok(v) => v,
            Err(_) => return Err(Error::JsonConversionFailed),
        };
        self.metadata = Some(json_val);
        Ok(self)
    }

    /// Sets the severity of the error.
    pub fn severity(mut self, val: Severity) -> Self {
        self.severity = Some(val);
        self
    }

    /// Sets the grouping hash for the Bugsnag web interface.
    pub fn grouping_hash(mut self, val: &'a str) -> Self {
        self.grouping_hash = Some(val);
        self
    }

    /// Call this function to explicitly send the notification to Bugsnag.
    /// This function will be called implicit if this object is dropped, but the notification will
    /// not be send twice.
    pub fn send(&mut self) -> Result<(), Error> {
        if self.send_executed {
            return Ok(());
        }

        self.send_executed = true;

        let json = self.create_json()?;
        self.bugsnag.send(&json, true)
    }

    /// Prepares the json as string
    fn create_json(&self) -> Result<String, Error> {
        let stacktrace = self.bugsnag.create_stacktrace(self.methods_to_ignore);
        let exceptions = vec![exception::Exception::new(
            self.error_class,
            self.message,
            &stacktrace,
        )];
        let metadata = {
            let ts = chrono::Utc::now().to_rfc3339();
            let json = if let Some(md) = &self.metadata {
                json!({
                    "bugsnag-rs": {"occurred": ts},
                    "metaData": md
                })
            } else {
                json!({"bugsnag-rs": {"occurred": ts}})
            };
            Some(json)
        };
        let events = vec![event::Event::new(
            &exceptions,
            self.severity.as_ref(),
            self.context,
            self.grouping_hash,
            &self.bugsnag.device_info,
            &self.bugsnag.app_info,
            &self.bugsnag.user,
            &metadata,
        )];
        let notification = notification::Notification::new(&events);

        match serde_json::to_string(&notification) {
            Ok(json) => Ok(json),
            Err(_) => Err(Error::JsonConversionFailed),
        }
    }
}

impl<'a, 'bugsnag> Drop for NotifyBuilder<'a, 'bugsnag> {
    fn drop(&mut self) {
        let _ = self.send();
    }
}

impl Bugsnag {
    /// Creates a new instance of the Bugsnag api
    pub fn new(api_key: &str, project_source_dir: &str) -> Bugsnag {
        Bugsnag {
            api_key: api_key.to_owned(),
            device_info: deviceinfo::DeviceInfo::generate(),
            user: None,
            app_info: None,
            project_source_dir: project_source_dir.to_owned(),
            offline_storage: None,
        }
    }

    /// Notifies the Bugsnag web-interface about an error.
    /// The function returns a builder to provide more information about the error.
    pub fn notify<'a, 'bugsnag>(
        &'bugsnag self,
        error_class: &'a str,
        message: &'a str,
    ) -> NotifyBuilder<'a, 'bugsnag> {
        NotifyBuilder::new(self, error_class, message)
    }

    fn create_stacktrace(&self, methods_to_ignore: Option<&[&str]>) -> Vec<stacktrace::Frame> {
        if let Some(ignore) = methods_to_ignore {
            let in_project_check = |file: &str, method: &str| {
                file.starts_with(self.project_source_dir.as_str())
                    && ignore.iter().any(|check| !method.contains(*check))
            };

            stacktrace::create_stacktrace(&in_project_check)
        } else {
            let in_project_check =
                |file: &str, _: &str| file.starts_with(self.project_source_dir.as_str());

            stacktrace::create_stacktrace(&in_project_check)
        }
    }

    /// Send a json string to the Bugsnag endpoint
    fn send(&self, json: &str, store_on_error: bool) -> Result<(), Error> {
        let client = reqwest::blocking::Client::new();
        let request = client
            .post(NOTIFY_URL)
            .body(json.to_string())
            .header("Content-Type", "application/json")
            .header("Bugsnag-Api-Key", self.api_key.clone())
            .header("Bugsnag-Payload-Version", notification::PAYLOAD_VERSION);
        match request.send() {
            Ok(_) => Ok(()),
            Err(_) => {
                if store_on_error {
                    let os = match &self.offline_storage {
                        Some(os) => os,
                        None => return Err(Error::JsonTransferAndStorageFailed),
                    };
                    let name = format!("{os}/{OFFLINE_REPORT_PREFIX}_{}", uuid::Uuid::new_v4());
                    if std::fs::write(name, json).is_err() {
                        return Err(Error::JsonTransferAndStorageFailed);
                    }
                }
                Err(Error::JsonTransferFailed)
            }
        }
    }

    /// Sets information about the device. These information will be send to
    /// Bugsnag when notify is called.
    pub fn set_device_info(&mut self, hostname: Option<&str>, version: Option<&str>) {
        if let Some(name) = hostname {
            self.device_info.set_hostname(name);
        }

        if let Some(ver) = version {
            self.device_info.set_os_version(ver);
        }
    }

    /// Sets information about the application that uses this api. These information
    /// will be send to Bugsnag when notify is called.
    pub fn set_app_info(
        &mut self,
        version: Option<&str>,
        release_stage: Option<&str>,
        atype: Option<&str>,
    ) {
        self.app_info = Some(appinfo::AppInfo::new(version, release_stage, atype));
    }

    pub fn reset_app_info(&mut self) {
        self.app_info = None;
    }

    pub fn set_user(&mut self, user: user::User) {
        self.user = Some(user);
    }

    pub fn use_offline_storage(&mut self, storage: &str) {
        self.offline_storage = Some(storage.to_string())
    }

    pub fn retry_from_storage(&self) -> Result<(), Error> {
        let os = match &self.offline_storage {
            Some(storage) => storage,
            None => return Err(Error::OfflineStorageError),
        };

        let entries = match std::fs::read_dir(os) {
            Ok(entries) => entries
                .flatten()
                .filter(|e| match e.file_name().to_str() {
                    Some(s) => s.starts_with(OFFLINE_REPORT_PREFIX),
                    None => false,
                })
                .collect::<Vec<DirEntry>>(),
            Err(_) => return Err(Error::OfflineStorageError),
        };

        for entry in entries {
            let report = match std::fs::read_to_string(entry.path()) {
                Ok(r) => r,
                Err(_) => return Err(Error::OfflineStorageError),
            };

            if let Err(e) = self.send(&report, false) {
                return Err(e);
            }
            std::fs::remove_file(entry.path()).ok();
        }
        Ok(())
    }

    pub fn get_project_source_dir(&self) -> &String {
        &self.project_source_dir
    }
}

#[cfg(test)]
mod tests {
    use super::Bugsnag;

    #[test]
    fn test_get_project_dir() {
        let api = Bugsnag::new("api-key", "my-dir");
        assert_eq!(api.get_project_source_dir(), "my-dir");
    }
}
