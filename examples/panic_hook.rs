use bugsnag::{Bugsnag, Severity};
use std::panic;

fn test(num: i32) -> i32 {
    i32::MAX + num
}

fn init_bugsnag() {
    let mut bugsnag = Bugsnag::new("api-key", env!("CARGO_MANIFEST_DIR"));

    bugsnag.set_app_info(
        Some(env!("CARGO_PKG_VERSION")),
        Some("development"),
        Some("rust"),
    );

    panic::set_hook(Box::new(move |info| {
        let message = bugsnag::panic::to_message(info);

        let mut bugsnag = bugsnag.clone();
        let res = bugsnag
            .notify("Panic", &message)
            .severity(Severity::Error)
            .send();

        if let Err(e) = res {
            println!("error sending panic report: {e}");
        }
    }));
}

fn main() {
    init_bugsnag();

    println!("{}", test(1));
}
