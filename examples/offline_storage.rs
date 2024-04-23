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

    // provide a path to a directory in which to store failed bug reports
    bugsnag.use_offline_storage("./");

    // at startup (or any other time) this can be called to try and retransmit
    // errors stored in offline storage
    // upon successful transmission, the local report will be deleted
    if let Err(e) = bugsnag.retry_from_storage() {
        println!("error retransmitting bug reports from offline storage: {e:?}");
    }

    if let Err(e) = test() {
        // when an offline storage was provided, failed bug reports will
        // be stored locally upon failed transmission
        // try disabling internet connection and then run this example
        let res = bugsnag
            .notify("Error", &format!("{e:?}"))
            .severity(Severity::Error)
            .send();

        if let Err(e) = res {
            println!("error transmitting bug report: {e:?}");
        }
    }

    // after re-enabling internet connection and running this example again
    // two errors will show up on bugsnag website:
    // 1) the locally stored one (when no internet connection was available)
    // 2) the regular report when send() was successful
    // have a look at "BUGSNAG RS" tab on bugsnag website
    // all bugsnag-rs reports have an "occurred" timestamp in this tab
}
