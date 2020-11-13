use serde::Deserialize;
use serde_json::value::Value;
use std::collections::HashMap;

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct Feat {
    #[serde(alias = "abilityIncreases")]
    ability_increases: Vec<String>,
    description: String,
    key: String,
    name: String,
    #[serde(alias = "optionPack")]
    option_pack: String,
    #[serde(alias = "pathPrereqs")]
    path_prereqs: FeatPathPrereqs,
    prereqs: Vec<String>,
    props: Value, // TODO: unorganized
}

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct FeatPathPrereqs {
    race: HashMap<String, bool>,
}
