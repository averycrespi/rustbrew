use serde::Deserialize;
use serde_json::value::Value;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Spell {
    #[serde(alias = "castingTime")]
    casting_time: String,

    #[serde(default)]
    components: Value, //TODO

    #[serde(default)]
    description: String,

    #[serde(default)]
    #[serde(alias = "disabled?")]
    disabled: bool,

    #[serde(default)]
    duration: String,

    #[serde(default)]
    #[serde(alias = "editEvent")]
    edit_event: Vec<Value>, //TODO

    key: String,
    level: u64,
    name: String,

    #[serde(alias = "optionPack")]
    option_pack: String,

    range: String,

    #[serde(default)]
    ritual: bool,

    school: String,

    #[serde(alias = "spellLists")]
    spell_lists: HashMap<String, bool>,
}
