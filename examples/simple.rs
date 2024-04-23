use bugsnag::{Bugsnag, Severity};
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

    if let Err(e) = test() {
        bugsnag
            .notify("Error", &format!("{e:?}"))
            .severity(Severity::Error)
            .send()
            .unwrap();
    }
}
