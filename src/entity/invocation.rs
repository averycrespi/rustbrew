use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct Invocation {
    #[serde(alias = "disabled?")]
    disabled: bool,
    description: String,
    key: String,
    name: String,
    #[serde(alias = "optionPack")]
    option_pack: String,
}
