#[macro_use]
extern crate lazy_static;

use std::time::{Duration, Instant};
mod parse_deckcode;
mod parse_json;
mod simulate;

fn main() {
    println!();
    let start = Instant::now();
    simulate::run();
    let duration: Duration = start.elapsed();
    println!("running time: {:.2?}", duration);
}
