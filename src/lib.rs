// Copyright (c) 2018 Joseph R. Quinn <quinn.josephr@protonmail.com>
// SPDX-License-Identifier: ISC

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
    fn test_is_dev() {
        let e: bool = is_dev!("ENV", "development");
        assert!(e);
    }
}
