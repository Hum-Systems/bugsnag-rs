use bugsnag::{Bugsnag, RateLimit, SendLimit, Severity};
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

    // rate limit the number of reports sent to 10 per hour and 100 per day, whichever is reached first

    bugsnag.rate_limit(RateLimit::new(
        vec![
            SendLimit::new(std::time::Duration::from_secs(3600), 10),
            SendLimit::new(std::time::Duration::from_secs(3600 * 24), 100),
        ],
        std::path::PathBuf::from("rate_limit.json"),
        None,
    ));

    if let Err(e) = test() {
        bugsnag
            .notify("Error", &format!("{e:?}"))
            .severity(Severity::Error)
            .send()
            .unwrap();
    }
}
