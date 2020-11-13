use serde::Deserialize;
use serde_json::value::Value;
use std::collections::HashMap;

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct Class {
    #[serde(alias = "abilityIncreaseLevels")]
    ability_increase_levels: Vec<u64>,
    #[serde(alias = "armorChoices")]
    armor_choices: Vec<ClassArmorChoice>,
    #[serde(alias = "disabled?")]
    disabled: bool,
    equipment: HashMap<String, u64>,
    help: String,
    #[serde(alias = "hitDie")]
    hit_die: u64,
    key: String,
    #[serde(alias = "levelModifiers")]
    level_modifiers: Vec<ClassLevelModifier>,
    #[serde(alias = "levelSelections")]
    level_selections: Vec<ClassLevelSelection>,
    name: String,
    nil: Value, // TODO: JS conversion artifact?
    #[serde(alias = "optionPack")]
    option_pack: String,
    profs: Value, // TODO: complicated
    spellcasting: ClassSpellcasting,
    #[serde(alias = "spellsKnown")]
    spells_known: HashMap<String, u64>,
    #[serde(alias = "subclassHelp")]
    subclass_help: String,
    #[serde(alias = "subclassLevel")]
    subclass_level: u64,
    #[serde(alias = "subclassTitle")]
    subclass_title: String,
    traits: Vec<ClassTrait>,
    #[serde(alias = "weaponChoices")]
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
    #[serde(alias = "cantripsKnown")]
    cantrips_known: HashMap<String, u64>,
    #[serde(alias = "cantrips?")]
    cantrips: bool,
    #[serde(alias = "knownMode")]
    known_mode: String,
    #[serde(alias = "levelFactor")]
    level_factor: u64,
    #[serde(alias = "spellList")]
    spell_list: HashMap<String, Vec<String>>,
    #[serde(alias = "spellsKnown")]
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
