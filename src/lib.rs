// Copyright (c) 2018 Joseph R. Quinn <quinn.josephr@protonmail.com>
// SPDX-License-Identifier: ISC

//! [![Build Status](https://travis-ci.org/quinnjr/is-dev-rs.svg?branch=master)](https://travis-ci.org/quinnjr/is-dev-rs)
//!
//! A simple macro to determine if an environment is
//! a "development" environment.
//!
//! Built because some of us (me) are lazy and don't feel like
//! copy-pasting the same check for development
//! environments over and over again.
//!
//! :heart:

#![allow(unused_imports)]
use std::env;
use std::ffi::OsStr;

#[allow(unused_macros)]
macro_rules! is_dev {
    ($var: expr, $m: expr) => {{
        match env::var_os($var)
        {
            Some(val) => val == OsStr::new($m),
            None => false
        }
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Only tests positive with ENV=development
    // set on the testing machine.
    fn test_is_dev() {
        let e: bool = is_dev!("ENV", "development");
        assert!(e)
    }

    #[test]
    fn test_is_not_dev() {
        let e: bool = is_dev!("NODE_ENV", "development");
        assert!(!e);
        let f: bool = is_dev!("SUPERSEKRITKEY", "development");
        assert!(!f)
    }
}
