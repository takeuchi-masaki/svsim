use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use std::{fs::File, io::Write};

const TOKEN_SET_ID: u32 = 90000;
const ROT_EXP_CNT: u32 = 5;

fn deserialize_null_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    T: Default + Deserialize<'de>,
    D: Deserializer<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}

#[derive(Serialize, Deserialize)]
pub struct JsonCard {
    pub card_id: u32,
    pub foil_card_id: u32,
    pub card_set_id: u32,
    #[serde(deserialize_with = "deserialize_null_default")]
    pub card_name: String,
    pub card_name_ruby: String,
    pub is_foil: u32,
    pub char_type: u32,
    pub clan: u32,
    pub tribe_name: String,
    pub skill: String,
    pub skill_condition: String,
    pub skill_target: String,
    pub skill_option: String,
    pub skill_preprocess: String,
    pub skill_disc: String,
    pub org_skill_disc: String,
    pub evo_skill_disc: String,
    pub org_evo_skill_disc: String,
    pub cost: u32,
    pub atk: u32,
    pub life: u32,
    pub evo_atk: u32,
    pub evo_life: u32,
    pub rarity: u32,
    pub get_red_ether: u32,
    pub use_red_ether: u32,
    pub description: String,
    pub evo_description: String,
    pub cv: String,
    #[serde(deserialize_with = "deserialize_null_default")]
    pub copyright: String,
    pub base_card_id: u32,
    pub normal_card_id: u32,
    pub format_type: u32,
    pub restricted_count: u32,
    pub restricted_count_co_main: u32,
    pub restricted_count_co_sub: u32,
}

pub fn get_all_cards() -> Vec<JsonCard> {
    let all_filename = "data/class0.json";
    let file = File::open(all_filename).expect("bad class0.json");
    let data: Value = serde_json::from_reader(file).expect("can't read json");
    let card_vals: Value = data["data"]["cards"].clone();
    let all_cards: Vec<JsonCard> =
        serde_json::from_value(card_vals).expect("type parse error");
    return all_cards;
}

pub fn get_rot_cards() -> Vec<JsonCard> {
    let rot_filename = "data/rot_cards.json";
    let file = File::open(rot_filename);
    if file.is_err() {
        let all_cards = get_all_cards();
        let mut mx_expac = 10000;

        // find current latest set - regular sets start with 1
        for card in &all_cards {
            if (card.card_set_id < TOKEN_SET_ID)
                && (card.card_set_id / 10000 == 1)
            {
                mx_expac = std::cmp::max(mx_expac, card.card_set_id);
            }
        }

        // get all current rotation cards && special cards
        let mut rot_card: Vec<JsonCard> = Vec::new();
        for card in all_cards {
            if ((card.card_set_id > mx_expac - ROT_EXP_CNT)
                && (card.card_set_id <= mx_expac))
                || (card.card_set_id / 10000 == 7)
                || (card.card_set_id == 10000)
            {
                rot_card.push(card);
            }
        }

        // save all the rot cards into rot_cards.json
        let json_str = serde_json::to_string_pretty(&rot_card)
            .expect("cannot deserialize Card");
        let mut rot_file =
            File::create(rot_filename).expect("can't create rot_cards.json");
        rot_file
            .write(json_str.as_bytes())
            .expect("bad rot file write");
        return rot_card;
    }

    let data: Value = serde_json::from_reader(file.expect("bad file"))
        .expect("can't read json");
    let card_vec: Vec<JsonCard> =
        serde_json::from_value(data).expect("type parse error");
    return card_vec;
}
