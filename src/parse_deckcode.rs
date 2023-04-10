use serde::{Deserialize, Serialize};
use std::collections::HashMap;

lazy_static! {
    static ref KEY: HashMap<char, usize> = {
        let mut map = HashMap::new();
        let keystr =
            "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz-_";
        for (i, c) in keystr.chars().enumerate() {
            map.insert(c, i);
        }
        map
    };
}

#[derive(Serialize, Deserialize)]
struct Resp {
    text: String,
    clan: String,
    hash: String,
    errors: Vec<String>,
}

fn cardhash_to_cardid(card_hash: String) -> usize {
    let mut id: usize = 0;
    for c in card_hash.chars() {
        id *= 64;
        id += KEY.get(&c).expect("no hash found");
    }
    return id;
}

fn deckhash_to_cardid_list(deck_hash: String) -> Vec<usize> {
    let mut cardid_list: Vec<usize> = Vec::new();
    let mut prev_hash = deck_hash.clone().split_off(4);
    for _ in 0..40 {
        let mut next_hash: String = prev_hash.clone();
        if prev_hash.len() > 5 {
            next_hash = prev_hash.split_off(5);
        }
        // println!("{}", prev_hash); // card hash
        cardid_list.push(cardhash_to_cardid(prev_hash));
        prev_hash = next_hash.split_off(1);
    }
    return cardid_list;
}

pub fn deckcode_to_decklist(deckcode: &str) -> Vec<usize> {
    assert!(deckcode.len() == 4);
    let url = "https://shadowverse-portal.com/api/v1/deck/import";
    let params = [("format", "json"), ("deck_code", &deckcode)];
    let url = reqwest::Url::parse_with_params(url, &params).expect("bad");
    let res = reqwest::blocking::get(url).expect("bad");
    let text = res.text().expect("bad text");
    let resp: serde_json::Value =
        serde_json::from_str(&text).expect("bad parse");
    // println!("{}", serde_json::to_string_pretty(&resp).expect("bad json"));
    let resp_data: Resp =
        serde_json::from_value(resp["data"].clone()).expect("bad json");
    let deck_hash: String = resp_data.hash;
    return deckhash_to_cardid_list(deck_hash);
    // return deck_hash; // FOR TESTING PURPOSES (also change return value)
}

#[cfg(test)]
mod tests {
    use crate::parse_deckcode::cardhash_to_cardid;

    #[test]
    fn cardhash() {
        assert_eq!(119121010, cardhash_to_cardid("76QHo".to_string()));
        assert_eq!(128621010, cardhash_to_cardid("7gfdI".to_string()));
    }

    // #[test]
    // fn deckcode() {
    //     // MAKE SURE TO UPDATE DECKCODE BEFORE TESTING
    //     // https://shadowverse-portal.com/deck/3.6.7gfdI.7gfdI.7gfdI.7gioQ.7gioQ.7gioQ.7ePaY.7ePaY.7ePaY.7ePqA.7ePqA.7ePqA.7Y-vC.7Y-vC.7Y-vC.7gfdS.7gfdS.7gfdS.7gi3Y.7gi3Y.7gi3Y.7gkVy.7gkVy.7gkVy.7RPoA.7RPoA.7RPoA.7RRVY.7RRVY.7gkVo.7gkVo.7gkVo.7RP3I.7RP3I.7VI56.7VI56.7VI56.7VI4o.7VI4o.7VI4o?lang=en
    //     let deckhash = "3.6.7gfdI.7gfdI.7gfdI.7gioQ.7gioQ.7gioQ.7ePaY.7ePaY.7ePaY.7ePqA.7ePqA.7ePqA.7Y-vC.7Y-vC.7Y-vC.7gfdS.7gfdS.7gfdS.7gi3Y.7gi3Y.7gi3Y.7gkVy.7gkVy.7gkVy.7RPoA.7RPoA.7RPoA.7RRVY.7RRVY.7gkVo.7gkVo.7gkVo.7RP3I.7RP3I.7VI56.7VI56.7VI56.7VI4o.7VI4o.7VI4o".to_string();
    //     assert_eq!(deckhash, deckcode_to_decklist("e9hr"));
    // }
}
