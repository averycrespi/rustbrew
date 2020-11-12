use serde::Deserialize;
use serde_json::value::Value;

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Selection {
    key: String,
    name: String,

    #[serde(alias = "optionPack")]
    option_pack: String,

    #[serde(default)]
    options: Vec<Value>, //TODO
}
