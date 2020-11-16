use serde::Deserialize;
use serde_json::value::Value;
use std::collections::HashMap;

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct Feat {
    ability_increases: Vec<String>,
    description: String,
    key: String,
    name: String,
    option_pack: String,
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
