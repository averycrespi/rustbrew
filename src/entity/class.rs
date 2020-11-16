use serde::Deserialize;
use serde_json::value::Value;
use std::collections::HashMap;

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct Class {
    ability_increase_levels: Vec<u64>,
    armor_choices: Vec<ClassArmorChoice>,
    #[serde(alias = "disabled?")]
    disabled: bool,
    equipment: HashMap<String, u64>,
    help: String,
    hit_die: u64,
    key: String,
    level_modifiers: Vec<ClassLevelModifier>,
    level_selections: Vec<ClassLevelSelection>,
    name: String,
    nil: Value, // TODO: JS conversion artifact?
    option_pack: String,
    profs: Value, // TODO: complicated
    spellcasting: ClassSpellcasting,
    spells_known: HashMap<String, u64>,
    subclass_help: String,
    subclass_level: u64,
    subclass_title: String,
    traits: Vec<ClassTrait>,
    weapon_choices: Vec<ClassWeaponChoice>,
    weapons: HashMap<String, u64>,
}

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct ClassArmorChoice {
    name: String,
    options: HashMap<String, u64>,
}

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct ClassLevelModifier {
    #[serde(alias = "type")]
    type_: String,
    value: String,
}

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct ClassLevelSelection {
    level: u64,
    num: u64,
    #[serde(alias = "type")]
    type_: String,
}

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct ClassSpellcasting {
    ability: String,
    cantrips_known: HashMap<String, u64>,
    #[serde(alias = "cantrips?")]
    cantrips: bool,
    known_mode: String,
    level_factor: u64,
    spell_list: HashMap<String, Vec<String>>,
    spells_known: HashMap<String, u64>,
}

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct ClassTrait {
    description: String,
    level: u64,
    name: String,
}

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct ClassWeaponChoice {
    name: String,
    options: HashMap<String, u64>,
}
