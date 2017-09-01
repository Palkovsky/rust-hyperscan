//! Hyperscan is a high-performance regular expression matching library.
//!
//! # Examples
//!
//! ```
//! #[macro_use]
//! extern crate hyperscan;
//!
//! use hyperscan::*;
//!
//! fn callback(id: u32, from: u64, to: u64, flags: u32, _: &BlockDatabase) -> u32 {
//!     assert_eq!(id, 0);
//!     assert_eq!(from, 5);
//!     assert_eq!(to, 9);
//!     assert_eq!(flags, 0);
//!
//!     println!("found pattern #{} @ [{}, {})", id, from, to);
//!
//!     0
//! }
//!
//! fn main() {
//!     let pattern = &pattern!{"test", flags => HS_FLAG_CASELESS|HS_FLAG_SOM_LEFTMOST};
//!     let db: BlockDatabase = pattern.build().unwrap();
//!     let scratch = db.alloc().unwrap();
//!
//!     db.scan::<BlockDatabase>("some test data", 0, &scratch, Some(callback), Some(&db)).unwrap();
//! }
//! ```

#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate error_chain;
extern crate libc;
#[macro_use]
extern crate log;
extern crate regex_syntax;

mod raw;
mod constants;
mod cptr;
#[macro_use]
pub mod errors;
mod api;
mod common;
#[macro_use]
mod compile;
mod runtime;

pub use constants::*;
pub use api::*;
pub use common::{valid_platform, version, BlockDatabase, RawDatabase, StreamingDatabase, VectoredDatabase};
pub use compile::{CompileFlags, Pattern, Patterns};
pub use runtime::{RawScratch, RawStream};

#[cfg(test)]
#[macro_use]
extern crate matches;
#[cfg(test)]
extern crate regex;

#[cfg(test)]
mod tests {
    pub use common::tests::*;
}
