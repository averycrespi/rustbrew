use serde::Deserialize;
use serde_json::value::Value;
use std::collections::HashMap;

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct Subclass {
    class: String,
    #[serde(alias = "clericSpells")]
    cleric_spells: Value, // TODO: class spells
    #[serde(alias = "disabled?")]
    disabled: bool,
    key: String,
    #[serde(alias = "levelModifiers")]
    level_modifiers: Vec<Value>, // TODO: complicated
    #[serde(alias = "levelSelections")]
    level_selections: Vec<SubclassLevelSelection>,
    name: String,
    #[serde(alias = "optionPack")]
    option_pack: String,
    #[serde(alias = "paladinSpells")]
    paladin_spells: Value, // TODO: class spells
    profs: Value, // TODO: complicated
    spellcasting: HashMap<String, u64>,
    #[serde(alias = "warlockSpells")]
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
