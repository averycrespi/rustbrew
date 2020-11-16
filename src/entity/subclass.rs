use serde::Deserialize;
use serde_json::value::Value;
use std::collections::HashMap;

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct Subclass {
    class: String,
    cleric_spells: Value, // TODO: class spells
    #[serde(alias = "disabled?")]
    disabled: bool,
    key: String,
    level_modifiers: Vec<Value>, // TODO: complicated
    level_selections: Vec<SubclassLevelSelection>,
    name: String,
    option_pack: String,
    paladin_spells: Value, // TODO: class spells
    profs: Value,          // TODO: complicated
    spellcasting: HashMap<String, u64>,
    warlock_spells: Value, // TODO: class spells
    traits: Vec<SubclassTrait>,
}

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct SubclassLevelSelection {
    level: u64,
    num: u64,
    #[serde(alias = "type")]
    type_: String,
}

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct SubclassTrait {
    description: String,
    level: u64,
    name: String,
    #[serde(alias = "type")]
    type_: String,
}
