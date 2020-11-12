use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Invocation {
    #[serde(default)]
    #[serde(alias = "disabled?")]
    disabled: bool,

    #[serde(default)]
    description: String,

    key: String,
    name: String,

    #[serde(alias = "optionPack")]
    option_pack: String,
}
