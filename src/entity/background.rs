use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct Background {
    equipment: HashMap<String, u64>,
    #[serde(alias = "equipmentChoices")]
    equipment_choices: Vec<BackgroundEquipmentChoice>,
    help: String,
    key: String,
    name: String,
    #[serde(alias = "optionPack")]
    option_pack: String,
    profs: BackgroundProfs,
    traits: Vec<BackgroundTrait>,
    treasure: HashMap<String, u64>,
}

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct BackgroundEquipmentChoice {
    name: String,
    options: HashMap<String, u64>,
}

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct BackgroundLanguageOptions {
    choose: u64,
    options: HashMap<String, bool>,
}

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct BackgroundProfs {
    skill: HashMap<String, bool>,
    tool: HashMap<String, bool>,
    #[serde(alias = "languageOptions")]
    language_options: BackgroundLanguageOptions,
    #[serde(alias = "toolOptions")]
    tool_options: HashMap<String, u64>,
}

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct BackgroundTrait {
    description: String,
    name: String,
    summary: String,
}
