[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE-MIT)

# bugsnag-rs
The Bugsnag api in rust.

# Example

```rust
use bugsnag;

let mut bugsnag = Bugsnag::new("api-key", env!("CARGO_MANIFEST_DIR"));

// setting the appinfo is not required, but recommended
bugsnag.set_app_info(
       Some(env!("CARGO_PKG_VERSION")),
       Some("development"),
       Some("rust"),
);

bugsnag.notify("Error", "this is a message from bugsnag-rs"))
       .severity(Severity::Error)
       .send()
       .unwrap();

```


For more examples on how to integrate bugsnag into a project, the examples folder provides some reference implementations.

# BugSnag API documentation

The structure of the json can be found [here](https://docs.bugsnag.com/api/error-reporting/).
