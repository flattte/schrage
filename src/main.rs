#![allow(unused)]
#[macro_use] extern crate serde;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate lazy_static;
mod mods;
#[allow(unused_imports)]
use mods::std_heaps::{schrage_heaps_std};
mod heap;
mod benchmark;
mod tests;

fn main() {
    println!("helo");
}
