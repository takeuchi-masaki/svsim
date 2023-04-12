#[macro_use]
extern crate lazy_static;

use std::time::{Duration, Instant};

use crate::parse_json::JsonCard;
mod parse_deckcode;
mod parse_json;
mod simulate;

fn main() {
    println!();
    let start = Instant::now();

    let cards: Vec<JsonCard> = parse_json::get_rot_cards();

    // get Vec<card_id>
    // https://shadowverse-portal.com/deck/3.2.7ThYI.7ThYI.7ThYI.7XTFM.7XTFM.7XTFM.7XY7Y.7XY7Y.7XY7Y.fskF2.fskF2.fskF2.7S_by.7S_by.7S_by.7PyHo.7PyHo.7PyHo.7XTF2.7XTF2.7XTF2.7XTFC.7XTFC.7XTFC.7XVhc.7XVhc.7XVhc.7Wj8w.7Wpko.7Wpko.7Wpko.7XVhI.7XVhI.7XVhI.7XaZo.7XaZo.7XaZo.gl-oS.gl-oS.gl-oS?lang=en
    let deckcode = "elhc";
    assert!(deckcode.len() == 4);
    let card_id_list: Vec<u32> =
        parse_deckcode::deckcode_to_decklist(&deckcode);

    simulate::run(cards, card_id_list);

    let duration: Duration = start.elapsed();
    println!("\nrunning time: {:.2?}", duration);
}
