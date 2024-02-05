#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![deny(clippy::cargo)]
#![forbid(unsafe_code)]
#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::items_after_statements)]
#![allow(clippy::multiple_crate_versions)]
#![allow(clippy::unused_async)]
#![doc = include_str!("../README.md")]

fn main() {
    println!("Hello, world!");
}
