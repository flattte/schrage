#![allow(unused)]
#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate lazy_static;

mod schrage;
#[allow(unused_imports)]
use schrage::std_heaps::schrage_heaps_std;
#[macro_use]
mod tests;

fn main() {
    println!("helo");
}
