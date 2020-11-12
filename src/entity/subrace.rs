use serde::Deserialize;
use serde_json::value::Value;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Subrace {
    #[serde(default)]
    abilities: HashMap<String, i64>,

    #[serde(default)]
    darkvision: u64,

    #[serde(default)]
    #[serde(alias = "disabled?")]
    disabled: bool,

    key: String,
    name: String,

    #[serde(alias = "optionPack")]
    option_pack: String,

    #[serde(default)]
    profs: Value, //TODO

    #[serde(default)]
    props: Value, //TODO

    #[serde(default)]
    race: String,

    #[serde(default)]
    speed: u64,

    #[serde(default)]
    spells: Vec<Value>, //TODO

    traits: Vec<Value>, //TODO
}
