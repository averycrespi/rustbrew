use serde::Deserialize;
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Language {
    #[serde(default)]
    description: String,

    key: String,
    name: String,

    #[serde(alias = "optionPack")]
    option_pack: String,
}
