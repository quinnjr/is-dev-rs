# is-dev-rs
[![Build Status](https://travis-ci.org/quinnjr/is-dev-rs.svg?branch=master)](https://travis-ci.org/quinnjr/is-dev-rs)

A simple macro to determine if an environment is
a "development" environment.

Built because some of us (me) are lazy and don't feel like
copy-pasting the same check for development
environments over and over again.

:heart:

Ex:
```rust
#[macro_use] extern crate is_dev;

fn test_is_dev() {
    let e: bool = is_dev!("ENV", "development");
    assert!(e);
}
```
