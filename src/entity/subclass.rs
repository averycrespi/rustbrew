use serde::Deserialize;
use serde_json::value::Value;

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Subclass {
    class: String,

    #[serde(default)]
    #[serde(alias = "clericSpells")]
    cleric_spells: Value, //TODO

    #[serde(default)]
    #[serde(alias = "disabled?")]
    disabled: bool,

    key: String,

    #[serde(alias = "levelModifiers")]
    level_modifiers: Value, //TODO

    #[serde(default)]
    #[serde(alias = "levelSelections")]
    level_selections: Value, //TODO

    name: String,
    #[serde(alias = "optionPack")]
    option_pack: String,

    #[serde(default)]
    #[serde(alias = "paladinSpells")]
    paladin_spells: Value, //TODO

    #[serde(default)]
    profs: Value, //TODO

    #[serde(default)]
    spellcasting: Value, //TODO

    #[serde(default)]
    #[serde(alias = "warlockSpells")]
    warlock_spells: Value, //TODO

    traits: Vec<SubclassTrait>,
}

#[derive(Deserialize, Debug)]
pub struct SubclassTrait {
    description: String,

    #[serde(default)]
    level: u64,

    #[serde(default)]
    name: String,
}
