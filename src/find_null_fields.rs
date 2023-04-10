// PARSE AND FIND NULL FIELDS WITHIN A JSON FILE
// use regex::Regex;
// use std::{collections::HashSet, fs::File};

// fn find_null_fields(json_str: &str) -> HashSet<String> {
//     let re = Regex::new(r#""(\w+)":\s*null"#).unwrap();
//     let mut null_fields: HashSet<String> = HashSet::new();
//     for capture in re.captures_iter(json_str) {
//         let key = capture[1].to_string();
//         null_fields.insert(key);
//     }
//     null_fields
// }

// fn main() {
//     let filename = "data/class0.json";
//     let file = File::open(filename).expect("bad file");
//     let val: serde_json::Value =
//         serde_json::from_reader(file).expect("can't read from file");

//     let null_fields = find_null_fields(
//         &serde_json::to_string(&val).expect("can't make string"),
//     );
//     println!("{:?}", null_fields); // prints ["card_name", "cv", "copyright"]
// }
