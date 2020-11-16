use serde::Deserialize;
use serde_json::value::Value;
use std::collections::HashMap;

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct Monster {
    alignment: String,
    armor_class: u64,
    armor_notes: String,
    cha: u64,
    challenge: f64,
    con: u64,
    condition_immunities: String,
    damage_immunities: String,
    damage_resistances: String,
    damage_vulnerabilities: String,
    description: String,
    dex: u64,
    hit_points: MonsterHitPoints,
    int: u64,
    key: String,
    languages: String,
    legendary_actions: MonsterLegendaryActions,
    name: String,
    option_pack: String,
    props: HashMap<String, Value>, // TODO: unorganized
    saving_throws: HashMap<String, Option<i64>>,
    size: String,
    senses: String,
    skills: HashMap<String, u64>,
    speed: String,
    str: u64,
    traits: Vec<MonsterTrait>,
    #[serde(alias = "type")]
    type_: String,
    wis: u64,
}

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct MonsterHitPoints {
    die: u64,
    die_count: u64,
    modifier: i64,
}

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct MonsterLegendaryActions {
    description: String,
}

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct MonsterTrait {
    description: String,
    name: String,
    #[serde(alias = "type")]
    type_: String,
}
