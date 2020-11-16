//! Types and methods for parsing Orcbrew entities.

use serde::Deserialize;
use serde_json::value::Value;
use std::collections::HashMap;
use std::convert::Into;

/// An Orcbrew entity.
#[derive(Debug)]
pub enum Entity {
    Background(Background),
    Class(Class),
    Feat(Feat),
    Invocation(Invocation),
    Language(Language),
    Monster(Monster),
    Race(Race),
    Selection(Selection),
    Spell(Spell),
    Subclass(Subclass),
    Subrace(Subrace),
}

/// A set of related Orcbrew entities.
///
/// Entities are grouped according to type.
#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct EntitySet {
    #[serde(alias = "disabled?")]
    disabled: bool,

    #[serde(alias = "orcpub.dnd.e5/backgrounds")]
    backgrounds: HashMap<String, Background>,

    #[serde(alias = "orcpub.dnd.e5/classes")]
    classes: HashMap<String, Class>,

    #[serde(alias = "orcpub.dnd.e5/feats")]
    feats: HashMap<String, Feat>,

    #[serde(alias = "orcpub.dnd.e5/invocations")]
    invocations: HashMap<String, Invocation>,

    #[serde(alias = "orcpub.dnd.e5/languages")]
    languages: HashMap<String, Language>,

    #[serde(alias = "orcpub.dnd.e5/monsters")]
    monsters: HashMap<String, Monster>,

    #[serde(alias = "orcpub.dnd.e5/races")]
    races: HashMap<String, Race>,

    #[serde(alias = "orcpub.dnd.e5/selections")]
    selections: HashMap<String, Selection>,

    #[serde(alias = "orcpub.dnd.e5/spells")]
    spells: HashMap<String, Spell>,

    #[serde(alias = "orcpub.dnd.e5/subclasses")]
    subclasses: HashMap<String, Subclass>,

    #[serde(alias = "orcpub.dnd.e5/subraces")]
    subraces: HashMap<String, Subrace>,
}

impl Into<Vec<Entity>> for EntitySet {
    /// Convert an entity set to a vector of entities.
    fn into(self) -> Vec<Entity> {
        let mut entities = vec![];
        for (_, background) in self.backgrounds {
            entities.push(Entity::Background(background));
        }
        for (_, class) in self.classes {
            entities.push(Entity::Class(class));
        }
        for (_, feat) in self.feats {
            entities.push(Entity::Feat(feat));
        }
        for (_, invocation) in self.invocations {
            entities.push(Entity::Invocation(invocation));
        }
        for (_, language) in self.languages {
            entities.push(Entity::Language(language));
        }
        for (_, monster) in self.monsters {
            entities.push(Entity::Monster(monster));
        }
        for (_, race) in self.races {
            entities.push(Entity::Race(race));
        }
        for (_, selection) in self.selections {
            entities.push(Entity::Selection(selection));
        }
        for (_, spell) in self.spells {
            entities.push(Entity::Spell(spell));
        }
        for (_, subclass) in self.subclasses {
            entities.push(Entity::Subclass(subclass));
        }
        for (_, subrace) in self.subraces {
            entities.push(Entity::Subrace(subrace));
        }
        entities
    }
}

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct Background {
    equipment: HashMap<String, u64>,
    equipment_choices: Vec<BackgroundEquipmentChoice>,
    help: String,
    key: String,
    name: String,
    option_pack: String,
    profs: BackgroundProfs,
    traits: Vec<BackgroundTrait>,
    treasure: HashMap<String, u64>,
}

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct BackgroundEquipmentChoice {
    name: String,
    options: HashMap<String, u64>,
}

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct BackgroundLanguageOptions {
    choose: u64,
    options: HashMap<String, bool>,
}

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct BackgroundProfs {
    skill: HashMap<String, bool>,
    tool: HashMap<String, bool>,
    language_options: BackgroundLanguageOptions,
    tool_options: HashMap<String, u64>,
}

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct BackgroundTrait {
    description: String,
    name: String,
    summary: String,
}

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct Class {
    ability_increase_levels: Vec<u64>,
    armor_choices: Vec<ClassArmorChoice>,
    #[serde(alias = "disabled?")]
    disabled: bool,
    equipment: HashMap<String, u64>,
    help: String,
    hit_die: u64,
    key: String,
    level_modifiers: Vec<ClassLevelModifier>,
    level_selections: Vec<ClassLevelSelection>,
    name: String,
    option_pack: String,
    profs: Value, // TODO: complicated
    spellcasting: ClassSpellcasting,
    spells_known: HashMap<String, u64>,
    subclass_help: String,
    subclass_level: u64,
    subclass_title: String,
    traits: Vec<ClassTrait>,
    weapon_choices: Vec<ClassWeaponChoice>,
    weapons: HashMap<String, u64>,
}

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct ClassArmorChoice {
    name: String,
    options: HashMap<String, u64>,
}

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct ClassLevelModifier {
    #[serde(alias = "type")]
    type_: String,
    value: String,
}

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct ClassLevelSelection {
    level: u64,
    num: u64,
    #[serde(alias = "type")]
    type_: String,
}

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct ClassSpellcasting {
    ability: String,
    cantrips_known: HashMap<String, u64>,
    #[serde(alias = "cantrips?")]
    cantrips: bool,
    known_mode: String,
    level_factor: u64,
    spell_list: HashMap<String, Vec<String>>,
    spells_known: HashMap<String, u64>,
}

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct ClassTrait {
    description: String,
    level: u64,
    name: String,
}

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct ClassWeaponChoice {
    name: String,
    options: HashMap<String, u64>,
}

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

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct Invocation {
    #[serde(alias = "disabled?")]
    disabled: bool,
    description: String,
    key: String,
    name: String,
    option_pack: String,
}

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct Language {
    description: String,
    key: String,
    name: String,
    option_pack: String,
}

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

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct Race {
    abilities: HashMap<String, i64>,
    darkvision: u64,
    #[serde(alias = "disabled?")]
    disabled: bool,
    help: String,
    key: String,
    languages: Vec<String>,
    name: String,
    option_pack: String,
    profs: Value, // TODO: complicated
    props: Value, // TODO: unorganized
    size: String,
    speed: u64,
    spells: Vec<RaceSpell>,
    traits: Vec<RaceTrait>,
}

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct RaceTrait {
    description: String,
    name: String,
    #[serde(alias = "type")]
    type_: String,
}

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct RaceSpell {
    level: u64,
    value: RaceSpellValue,
}

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct RaceSpellValue {
    ability: String,
    key: String,
    level: u64,
}

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

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct Spell {
    casting_time: String,
    components: SpellComponents,
    description: String,
    #[serde(alias = "disabled?")]
    disabled: bool,
    duration: String,
    edit_event: Vec<Value>, // TODO: React?
    key: String,
    level: u64,
    name: String,
    option_pack: String,
    range: String,
    ritual: bool,
    school: String,
    spell_lists: HashMap<String, bool>,
}

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct SpellComponents {
    material: bool,
    material_component: String,
    somatic: bool,
    verbal: bool,
}

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct Subclass {
    class: String,
    cleric_spells: Value, // TODO: class spells
    #[serde(alias = "disabled?")]
    disabled: bool,
    key: String,
    level_modifiers: Vec<Value>, // TODO: complicated
    level_selections: Vec<SubclassLevelSelection>,
    name: String,
    option_pack: String,
    paladin_spells: Value, // TODO: class spells
    profs: Value,          // TODO: complicated
    spellcasting: HashMap<String, u64>,
    warlock_spells: Value, // TODO: class spells
    traits: Vec<SubclassTrait>,
}

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct SubclassLevelSelection {
    level: u64,
    num: u64,
    #[serde(alias = "type")]
    type_: String,
}

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct SubclassTrait {
    description: String,
    level: u64,
    name: String,
    #[serde(alias = "type")]
    type_: String,
}

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct Subrace {
    abilities: HashMap<String, i64>,
    darkvision: u64,
    #[serde(alias = "disabled?")]
    disabled: bool,
    key: String,
    name: String,
    option_pack: String,
    profs: Value, // TODO: complicated
    props: Value, // TODO: unorganized
    race: String,
    speed: u64,
    spells: Vec<SubraceSpell>,
    traits: Vec<SubraceTrait>,
}

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct SubraceSpell {
    level: u64,
    value: SubraceSpellValue,
}

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct SubraceSpellValue {
    ability: String,
    key: String,
    level: u64,
}

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct SubraceTrait {
    description: String,
    name: String,
}
