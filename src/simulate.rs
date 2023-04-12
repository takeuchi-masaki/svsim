use std::collections::HashMap;

use crate::parse_json::JsonCard;

// TODO: change card_id_list to deck from manage_decks
pub fn run(all_cards: Vec<JsonCard>, card_id_list: Vec<u32>) {
    // parse full card list
    let mut id_to_card: HashMap<u32, JsonCard> = HashMap::new();
    for card in all_cards {
        id_to_card.insert(card.card_id, card);
    }

    // print decklist
    println!("decklist: ");
    for id in card_id_list {
        let c = id_to_card.get(&id).expect("no card");
        if c.base_card_id != c.card_id {
            let base = id_to_card.get(&c.base_card_id);
            if base.is_none() {
                println!("no base card found for : {}", c.card_name);
                return;
            }
            println!("{}", base.expect("never throws").card_name);
        } else {
            println!("{}", c.card_name);
        }
    }
}
