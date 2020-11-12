use serde::Deserialize;
use serde_json::value::Value;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Class {
    #[serde(alias = "abilityIncreaseLevels")]
    ability_increase_levels: Vec<u64>,

    #[serde(default)]
    #[serde(alias = "armorChoices")]
    armor_choices: Vec<Value>, //TODO

    #[serde(default)]
    #[serde(alias = "disabled?")]
    disabled: bool,

    #[serde(default)]
    equipment: HashMap<String, u64>,

    #[serde(default)]
    help: String,

    #[serde(alias = "hitDie")]
    hit_die: u64,

    key: String,

    #[serde(alias = "levelModifiers")]
    level_modifiers: Vec<ClassLevelModifier>,

    #[serde(alias = "levelSelections")]
    level_selections: Vec<ClassLevelSelection>,

    name: String,

    nil: Value, //TODO

    #[serde(alias = "optionPack")]
    option_pack: String,

    profs: Value,        //TODO
    spellcasting: Value, //TODO

    #[serde(default)]
    #[serde(alias = "spellsKnown")]
    spells_known: HashMap<String, u64>,

    #[serde(default)]
    #[serde(alias = "subclassHelp")]
    subclass_help: String,

    #[serde(default)]
    #[serde(alias = "subclassLevel")]
    subclass_level: u64,

    #[serde(default)]
    #[serde(alias = "subclassTitle")]
    subclass_title: String,

    traits: Vec<Value>, //TODO

    #[serde(default)]
    #[serde(alias = "weaponChoices")]
    weapon_choices: Vec<Value>, //TODO

    #[serde(default)]
    weapons: HashMap<String, u64>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ClassLevelModifier {
    #[serde(alias = "type")]
    type_: String,

    value: String,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ClassLevelSelection {
    #[serde(default)]
    level: u64,

    #[serde(default)]
    num: u64,

    #[serde(alias = "type")]
    type_: String,
}
