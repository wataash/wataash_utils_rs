// SPDX-License-Identifier: Apache-2.0

//! # wataash_utils
//!
//! wataash's personal utilities.

#[doc(hidden)] // XXX: appears CLion's completion -- this doesn't make sense?
pub mod logger;

pub fn hello() {
    println!("hello");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
