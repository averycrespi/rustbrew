use serde::Deserialize;
use serde_json::value::Value;

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Feat {
    #[serde(alias = "abilityIncreases")]
    ability_increases: Vec<String>,

    #[serde(default)]
    description: String,

    key: String,
    name: String,

    #[serde(alias = "optionPack")]
    option_pack: String,

    #[serde(default)]
    #[serde(alias = "pathPrereqs")]
    path_prereqs: Value, //TODO

    prereqs: Vec<String>,

    #[serde(default)]
    props: Value, //TODO
}
