use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use std::{fs::File, io::Write};

const TOKEN_SET_ID: i32 = 90000;
const ROT_EXP_CNT: i32 = 5;

fn deserialize_null_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    T: Default + Deserialize<'de>,
    D: Deserializer<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}

#[derive(Serialize, Deserialize)]
pub struct Card {
    pub card_id: usize,
    pub foil_card_id: i32,
    pub card_set_id: i32,
    #[serde(deserialize_with = "deserialize_null_default")]
    pub card_name: String,
    pub card_name_ruby: String,
    pub is_foil: i32,
    pub char_type: i32,
    pub clan: i32,
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
    pub cost: i32,
    pub atk: i32,
    pub life: i32,
    pub evo_atk: i32,
    pub evo_life: i32,
    pub rarity: i32,
    pub get_red_ether: i32,
    pub use_red_ether: i32,
    pub description: String,
    pub evo_description: String,
    pub cv: String,
    #[serde(deserialize_with = "deserialize_null_default")]
    pub copyright: String,
    pub base_card_id: usize,
    pub normal_card_id: i32,
    pub format_type: i32,
    pub restricted_count: i32,
    pub restricted_count_co_main: i32,
    pub restricted_count_co_sub: i32,
}

pub fn get_rot_cards() -> Vec<Card> {
    let rot_filename = "data/rot_cards.json";
    let file = File::open(rot_filename);
    if file.is_err() {
        let all_filename = "data/class0.json";
        let file2 = File::open(all_filename).expect("bad class0.json");
        let data: Value =
            serde_json::from_reader(file2).expect("can't read json");
        let card_vals: Value = data["data"]["cards"].clone();
        let all_cards: Vec<Card> =
            serde_json::from_value(card_vals).expect("type parse error");
        let mut mx_expac = 10000;
        for card in &all_cards {
            if (card.card_set_id < TOKEN_SET_ID)
                && (card.card_set_id / 10000 == 1)
            {
                mx_expac = std::cmp::max(mx_expac, card.card_set_id);
            }
        }
        let mut rot_card: Vec<Card> = Vec::new();
        for card in all_cards {
            if (card.card_set_id > mx_expac - ROT_EXP_CNT)
                && (card.card_set_id <= mx_expac)
            {
                rot_card.push(card);
            }
        }
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
    let card_vec: Vec<Card> =
        serde_json::from_value(data).expect("type parse error");
    return card_vec;
}
