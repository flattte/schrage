#[macro_use] extern crate serde;
#[macro_use] extern crate serde_json;

mod mods;
#[allow(unused_imports)]
use mods::std_heaps::{SchrageContextBH, schrage_heaps_bh};
mod heap;


fn main() {
    println!("helo");
}
