use serde::Deserialize;
use serde_json::value::Value;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Monster {
    alignment: String,

    #[serde(alias = "armorClass")]
    armor_class: u64,

    #[serde(default)]
    #[serde(alias = "armorNotes")]
    armor_notes: String,

    cha: u64,

    #[serde(default)]
    challenge: f64,

    con: u64,

    #[serde(default)]
    #[serde(alias = "conditionImmunities")]
    condition_immunities: String,

    #[serde(default)]
    #[serde(alias = "damageImmunities")]
    damage_immunities: String,

    #[serde(default)]
    #[serde(alias = "damageResistances")]
    damage_resistances: String,

    #[serde(default)]
    #[serde(alias = "damageVulnerabilities")]
    damage_vulnerabilities: String,

    #[serde(default)]
    description: String,

    dex: u64,

    #[serde(alias = "hitPoints")]
    hit_points: MonsterHitPoints,

    int: u64,
    key: String,

    #[serde(default)]
    languages: String,

    #[serde(default)]
    #[serde(alias = "legendaryActions")]
    legendary_actions: Value, //TODO

    name: String,

    #[serde(alias = "optionPack")]
    option_pack: String,

    #[serde(default)]
    props: HashMap<String, Value>, //TODO

    #[serde(default)]
    #[serde(alias = "savingThrows")]
    saving_throws: HashMap<String, Value>, //TODO

    size: String,

    #[serde(default)]
    senses: String,

    #[serde(default)]
    skills: HashMap<String, u64>,

    speed: String,
    str: u64,
    traits: Vec<MonsterTrait>,

    #[serde(alias = "type")]
    type_: String,

    wis: u64,
}

#[derive(Deserialize, Debug)]
pub struct MonsterHitPoints {
    die: u64,

    #[serde(alias = "dieCount")]
    die_count: u64,

    #[serde(default)]
    modifier: i64,
}

#[derive(Deserialize, Debug)]
pub struct MonsterTrait {
    #[serde(default)]
    description: String,

    #[serde(default)]
    name: String,

    #[serde(default)]
    #[serde(alias = "type")]
    type_: String,
}
