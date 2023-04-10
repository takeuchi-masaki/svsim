use std::collections::HashMap;

use crate::{parse_deckcode, parse_json};

pub fn run() {
    // https://shadowverse-portal.com/deck/3.6.7gfdI.7gfdI.7gfdI.7gioQ.7gioQ.7gioQ.7ePaY.7ePaY.7ePaY.7ePqA.7ePqA.7ePqA.7Y-vC.7Y-vC.7Y-vC.7gfdS.7gfdS.7gfdS.7gi3Y.7gi3Y.7gi3Y.7gkVy.7gkVy.7gkVy.7RPoA.7RPoA.7RPoA.7RRVY.7RRVY.7gkVo.7gkVo.7gkVo.7RP3I.7RP3I.7VI56.7VI56.7VI56.7VI4o.7VI4o.7VI4o?lang=en
    let deckcode = "efa7";
    assert!(deckcode.len() == 4);

    // parse full card list
    let mut id_to_cardname: HashMap<usize, String> = HashMap::new();
    let mut cardname_to_id: HashMap<String, usize> = HashMap::new();
    let cards: Vec<parse_json::Card> = parse_json::get_rot_cards();
    for card in cards {
        id_to_cardname.insert(card.base_card_id, card.card_name.clone());
        cardname_to_id.insert(card.card_name, card.base_card_id);
    }

    // get Vec<card_id>
    let card_id_list: Vec<usize> =
        parse_deckcode::deckcode_to_decklist(&deckcode);

    // print decklist
    println!("decklist: ");
    for card in card_id_list {
        println!("{}", id_to_cardname.get(&card).expect("no card"));
    }
}
