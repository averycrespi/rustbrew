use serde::Deserialize;
use std::collections::HashMap;

mod background;
mod class;
mod feat;
mod invocation;
mod language;
mod monster;
mod race;
mod selection;
mod spell;
mod subclass;
mod subrace;

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct EntitySource {
    #[serde(default)]
    #[serde(alias = "disabled?")]
    disabled: bool,

    #[serde(default)]
    #[serde(alias = "orcpubDndE5/backgrounds")]
    backgrounds: HashMap<String, background::Background>,

    #[serde(default)]
    #[serde(alias = "orcpubDndE5/classes")]
    classes: HashMap<String, class::Class>,

    #[serde(default)]
    #[serde(alias = "orcpubDndE5/feats")]
    feats: HashMap<String, feat::Feat>,

    #[serde(default)]
    #[serde(alias = "orcpubDndE5/invocations")]
    invocations: HashMap<String, invocation::Invocation>,

    #[serde(default)]
    #[serde(alias = "orcpubDndE5/languages")]
    languages: HashMap<String, language::Language>,

    #[serde(default)]
    #[serde(alias = "orcpubDndE5/monsters")]
    monsters: HashMap<String, monster::Monster>,

    #[serde(default)]
    #[serde(alias = "orcpubDndE5/races")]
    races: HashMap<String, race::Race>,

    #[serde(default)]
    #[serde(alias = "orcpubDndE5/selections")]
    selections: HashMap<String, selection::Selection>,

    #[serde(default)]
    #[serde(alias = "orcpubDndE5/spells")]
    spells: HashMap<String, spell::Spell>,

    #[serde(default)]
    #[serde(alias = "orcpubDndE5/subclasses")]
    subclasses: HashMap<String, subclass::Subclass>,

    #[serde(default)]
    #[serde(alias = "orcpubDndE5/subraces")]
    subraces: HashMap<String, subrace::Subrace>,
}
