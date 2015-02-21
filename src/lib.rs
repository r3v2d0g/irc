//! A simple, thread-safe IRC library.
#![crate_name = "irc"]
#![crate_type = "lib"]
#![unstable]
#![warn(missing_docs)]

#![feature(collections, core, io, old_io, old_path)]
#[cfg(feature = "ctcp")] extern crate time;
#[cfg(feature = "encode")] extern crate encoding;
extern crate "rustc-serialize" as rustc_serialize;
#[cfg(feature = "ssl")] extern crate openssl;

pub mod client;
pub mod server;
