use serde::Deserialize;
use serde_json::value::Value;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Background {
    #[serde(default)]
    equipment: HashMap<String, u64>,

    #[serde(default)]
    #[serde(alias = "equipmentChoices")]
    equipment_choices: Vec<Value>, //TODO

    #[serde(default)]
    help: String,

    key: String,
    name: String,

    #[serde(alias = "optionPack")]
    option_pack: String,

    #[serde(default)]
    profs: BackgroundProfs,

    traits: Vec<Value>, //TODO

    #[serde(default)]
    treasure: HashMap<String, u64>,
}

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct BackgroundProfs {
    #[serde(default)]
    skill: HashMap<String, bool>,

    #[serde(default)]
    tool: HashMap<String, bool>,

    #[serde(default)]
    #[serde(alias = "languageOptions")]
    language_options: Value, //TODO

    #[serde(default)]
    #[serde(alias = "toolOptions")]
    tool_options: Value, //TODO
}
