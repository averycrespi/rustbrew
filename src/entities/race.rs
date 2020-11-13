use serde::Deserialize;
use serde_json::value::Value;
use std::collections::HashMap;

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct Race {
    abilities: HashMap<String, i64>,
    darkvision: u64,
    #[serde(alias = "disabled?")]
    disabled: bool,
    help: String,
    key: String,
    languages: Vec<String>,
    name: String,
    #[serde(alias = "optionPack")]
    option_pack: String,
    profs: Value, // TODO: complicated
    props: Value, // TODO: unorganized
    size: String,
    speed: u64,
    spells: Vec<RaceSpell>,
    traits: Vec<RaceTrait>,
}

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct RaceTrait {
    description: String,
    name: String,
    #[serde(alias = "type")]
    type_: String,
}

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct RaceSpell {
    level: u64,
    value: RaceSpellValue,
}

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct RaceSpellValue {
    ability: String,
    key: String,
    level: u64,
}
