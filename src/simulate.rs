use std::collections::HashMap;

use crate::{parse_deckcode, parse_json};

pub fn run() {
    // https://shadowverse-portal.com/deck/3.6.7gfdI.7gfdI.7gfdI.7gioQ.7gioQ.7gioQ.7ePaY.7ePaY.7ePaY.7ePqA.7ePqA.7ePqA.7Y-vC.7Y-vC.7Y-vC.7gfdS.7gfdS.7gfdS.7gi3Y.7gi3Y.7gi3Y.7gkVy.7gkVy.7gkVy.7RPoA.7RPoA.7RPoA.7RRVY.7RRVY.7gkVo.7gkVo.7gkVo.7RP3I.7RP3I.7VI56.7VI56.7VI56.7VI4o.7VI4o.7VI4o?lang=en
    let deckcode = "efj8";
    assert!(deckcode.len() == 4);

    // parse full card list
    let mut id_to_card: HashMap<usize, parse_json::Card> = HashMap::new();
    let cards: Vec<parse_json::Card> = parse_json::get_rot_cards();
    for card in cards {
        id_to_card.insert(card.card_id, card);
    }

    // get Vec<card_id>
    let card_id_list: Vec<usize> =
        parse_deckcode::deckcode_to_decklist(&deckcode);

    // print decklist
    println!("decklist: ");
    for id in card_id_list {
        let c = id_to_card.get(&id).expect("no card");
        if c.base_card_id != c.card_id {
            let base = id_to_card.get(&c.base_card_id).expect("no base card");
            println!("{}", base.card_name);
        } else {
            println!("{}", c.card_name);
        }
    }
}
