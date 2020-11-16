use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct Selection {
    key: String,
    name: String,
    option_pack: String,
    options: Vec<SelectionOption>,
}

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct SelectionOption {
    description: String,
    name: String,
}
