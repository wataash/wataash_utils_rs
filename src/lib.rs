// SPDX-License-Identifier: Apache-2.0

//! # wataash_utils
//!
//! wataash's personal utilities.

#[doc(hidden)] // XXX: appears CLion's completion -- this doesn't make sense?
pub mod logger;

// -------------------------------------------------------------------------------------------------
// misc

pub fn home_dir() -> String {
    // https://stackoverflow.com/q/37388107/4085441
    let home = dirs::home_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap();
    return home;
}

// -------------------------------------------------------------------------------------------------
// test

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
