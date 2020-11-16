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

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct EntitySource {
    #[serde(alias = "disabled?")]
    disabled: bool,

    #[serde(alias = "orcpubDndE5/backgrounds")]
    backgrounds: HashMap<String, background::Background>,

    #[serde(alias = "orcpubDndE5/classes")]
    classes: HashMap<String, class::Class>,

    #[serde(alias = "orcpubDndE5/feats")]
    feats: HashMap<String, feat::Feat>,

    #[serde(alias = "orcpubDndE5/invocations")]
    invocations: HashMap<String, invocation::Invocation>,

    #[serde(alias = "orcpubDndE5/languages")]
    languages: HashMap<String, language::Language>,

    #[serde(alias = "orcpubDndE5/monsters")]
    monsters: HashMap<String, monster::Monster>,

    #[serde(alias = "orcpubDndE5/races")]
    races: HashMap<String, race::Race>,

    #[serde(alias = "orcpubDndE5/selections")]
    selections: HashMap<String, selection::Selection>,

    #[serde(alias = "orcpubDndE5/spells")]
    spells: HashMap<String, spell::Spell>,

    #[serde(alias = "orcpubDndE5/subclasses")]
    subclasses: HashMap<String, subclass::Subclass>,

    #[serde(alias = "orcpubDndE5/subraces")]
    subraces: HashMap<String, subrace::Subrace>,
}
