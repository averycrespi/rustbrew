use serde::Deserialize;
use serde_json::value::Value;
use std::collections::HashMap;

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct Subrace {
    abilities: HashMap<String, i64>,
    darkvision: u64,
    #[serde(alias = "disabled?")]
    disabled: bool,
    key: String,
    name: String,
    #[serde(alias = "optionPack")]
    option_pack: String,
    profs: Value, // TODO: complicated
    props: Value, // TODO: unorganized
    race: String,
    speed: u64,
    spells: Vec<SubraceSpell>,
    traits: Vec<SubraceTrait>,
}

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct SubraceSpell {
    level: u64,
    value: SubraceSpellValue,
}

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct SubraceSpellValue {
    ability: String,
    key: String,
    level: u64,
}

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct SubraceTrait {
    description: String,
    name: String,
}
