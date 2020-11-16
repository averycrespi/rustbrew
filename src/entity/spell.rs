use serde::Deserialize;
use serde_json::value::Value;
use std::collections::HashMap;

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct Spell {
    casting_time: String,
    components: SpellComponents,
    description: String,
    #[serde(alias = "disabled?")]
    disabled: bool,
    duration: String,
    edit_event: Vec<Value>, // TODO: React?
    key: String,
    level: u64,
    name: String,
    option_pack: String,
    range: String,
    ritual: bool,
    school: String,
    spell_lists: HashMap<String, bool>,
}

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct SpellComponents {
    material: bool,
    material_component: String,
    somatic: bool,
    verbal: bool,
}
