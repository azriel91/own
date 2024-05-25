#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

//! Wraps an owned or borrowed value.
//!
//! This allows you to hold either an owned or borrowed value, allowing your
//! data type to be either `'static` or `'temporary`.
//!
//! There is a `OwnedOrRef::reborrow` method that allows the value to be
//! borrowed with a shorter lifetime.
//!
//!
//! # Usage
//!
//! Add the following to `Cargo.toml`
//!
//! ```toml
//! own = "0.1.1"
//! ```
//!
//! # Examples
//!
//! ```rust
//! use own::OwnedOrRef;
//!
//! fn print(name: OwnedOrRef<'_, String>) {
//!     println!("{}", *name);
//! }
//!
//! fn main() {
//!     let name = String::from("owned");
//!     print(name.into());
//!
//!     let name = &String::from("borrowed");
//!     print(name.into());
//! }
//! ```
//!
//! ```rust
//! use own::OwnedOrMutRef;
//!
//! fn make_uppercase(name: &mut OwnedOrMutRef<'_, String>) {
//!     name.make_ascii_uppercase();
//! }
//!
//! fn main() {
//!     let name = String::from("owned");
//!     let mut owned = name.into();
//!     make_uppercase(&mut owned);
//!     assert_eq!("OWNED", *owned);
//!
//!     let name = &mut String::from("borrowed");
//!     let mut borrowed = name.into();
//!     make_uppercase(&mut borrowed);
//!     assert_eq!("BORROWED", *borrowed);
//! }
//! ```
//!
//!
//! ## License
//!
//! Licensed under either of
//!
//! * Apache License, Version 2.0, ([LICENSE-APACHE] or <https://www.apache.org/licenses/LICENSE-2.0>)
//! * MIT license ([LICENSE-MIT] or <https://opensource.org/licenses/MIT>)
//!
//! at your option.
//!
//!
//! ### Contribution
//!
//! Unless you explicitly state otherwise, any contribution intentionally
//! submitted for inclusion in the work by you, as defined in the Apache-2.0
//! license, shall be dual licensed as above, without any additional terms or
//! conditions.
//!
//! [LICENSE-APACHE]: LICENSE-APACHE
//! [LICENSE-MIT]: LICENSE-MIT

pub use crate::{owned_or_mut_ref::OwnedOrMutRef, owned_or_ref::OwnedOrRef};

mod owned_or_mut_ref;
mod owned_or_ref;
