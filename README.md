[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE-MIT)

# bugsnag-rs
The Bugsnag api in rust.

# Example

```rust
use bugsnag;
let mut api = bugsnag::Bugsnag::new("api-key", env!("CARGO_MANIFEST_DIR"));

// setting the appinfo is not required, but recommended
api.set_app_info(Some(env!("CARGO_PKG_VERSION")),
                 Some("development"),
                 Some("rust"));

api.notify("Info", "This is a message from the rust bugsnag api.")
       .severity(bugsnag::Severity::Info);
```

Or in a panic handler you could do the following:

```rust

use bugsnag;
let mut api = bugsnag::Bugsnag::new("api-key", env!("CARGO_MANIFEST_DIR"));

bugsnag::panic::handle(&api, panic_info, None);

```

For more examples on how to integrate bugsnag into a project, the examples folder provides some reference implementations.


# Which json fields are missing?
- context (present but not exposed / usable)
- metaData

The structure of the json can be found [here](https://docs.bugsnag.com/api/error-reporting/).
