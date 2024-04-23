use bugsnag::{user::User, Bugsnag, Severity};
use serde_json::json;
use std::num::ParseIntError;

fn test() -> Result<i32, ParseIntError> {
    "test".parse::<i32>()
}

fn main() {
    let mut bugsnag = Bugsnag::new("api-key", env!("CARGO_MANIFEST_DIR"));

    bugsnag.set_app_info(
        Some(env!("CARGO_PKG_VERSION")),
        Some("development"),
        Some("rust"),
    );

    // setting user struct -> will be displayed in its own tab on bugsnag website
    bugsnag.set_user(User::new("123456789", "testuser", "test@user.com"));

    if let Err(e) = test() {
        bugsnag
            .notify("Error", &format!("{e:?}"))
            .severity(Severity::Error)
            // adding a context to this error report
            .context("main::test")
            // adding additional metadata -> will also be displayed in its own tab on bugsnag website
            .metadata(&json!({"ip_addr": "123.456.789.10"}))
            .unwrap()
            .send()
            .unwrap();
    }
}
