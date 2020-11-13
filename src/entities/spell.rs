use serde::Deserialize;
use serde_json::value::Value;
use std::collections::HashMap;

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct Spell {
    #[serde(alias = "castingTime")]
    casting_time: String,
    components: SpellComponents,
    description: String,
    #[serde(alias = "disabled?")]
    disabled: bool,
    duration: String,
    #[serde(alias = "editEvent")]
    edit_event: Vec<Value>, // TODO: React?
    key: String,
    level: u64,
    name: String,
    #[serde(alias = "optionPack")]
    option_pack: String,
    range: String,
    ritual: bool,
    school: String,
    #[serde(alias = "spellLists")]
    spell_lists: HashMap<String, bool>,
}

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct SpellComponents {
    material: bool,
    #[serde(alias = "materialComponent")]
    material_component: String,
    somatic: bool,
    verbal: bool,
}
