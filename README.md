# Application Boilerplate

This repo is just a small boilerplate project that can be used to make various sample applications demonstrating different rendering techniques, etc.

It is not associated with any specific graphics backend.

New binaries go in `bin` and can be run with `cargo run -r --bin appName`.

Any shared supporting code can be a new module in the `support` library.

Implement the `Application` trait from the `support` crate and then run the application with the `run` function!
