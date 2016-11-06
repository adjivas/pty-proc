#![feature(slice_patterns)]
#![feature(advanced_slice_patterns)]
#![feature(integer_atomics)]
#![feature(range_contains)]

#![crate_type= "lib"]
#![cfg_attr(feature = "nightly", feature(plugin))]
#![cfg_attr(feature = "lints", plugin(clippy))]
#![cfg_attr(feature = "lints", deny(warnings))]
#![cfg_attr(not(any(feature = "lints", feature = "nightly")), deny())]
#![deny(
    unused_import_braces,
    unused_qualifications
)]

#[macro_use(chan_select)] extern crate chan;
extern crate pty;
extern crate libc;
extern crate time;
extern crate errno;
extern crate sysinfo;

#[macro_use]
mod macros;
mod fork;
pub mod shell;
pub mod prelude;
