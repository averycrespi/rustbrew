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
pub struct EntitySet {
    #[serde(alias = "disabled?")]
    disabled: bool,

    #[serde(alias = "orcpub.dnd.e5/backgrounds")]
    backgrounds: HashMap<String, background::Background>,

    #[serde(alias = "orcpub.dnd.e5/classes")]
    classes: HashMap<String, class::Class>,

    #[serde(alias = "orcpub.dnd.e5/feats")]
    feats: HashMap<String, feat::Feat>,

    #[serde(alias = "orcpub.dnd.e5/invocations")]
    invocations: HashMap<String, invocation::Invocation>,

    #[serde(alias = "orcpub.dnd.e5/languages")]
    languages: HashMap<String, language::Language>,

    #[serde(alias = "orcpub.dnd.e5/monsters")]
    monsters: HashMap<String, monster::Monster>,

    #[serde(alias = "orcpub.dnd.e5/races")]
    races: HashMap<String, race::Race>,

    #[serde(alias = "orcpub.dnd.e5/selections")]
    selections: HashMap<String, selection::Selection>,

    #[serde(alias = "orcpub.dnd.e5/spells")]
    spells: HashMap<String, spell::Spell>,

    #[serde(alias = "orcpub.dnd.e5/subclasses")]
    subclasses: HashMap<String, subclass::Subclass>,

    #[serde(alias = "orcpub.dnd.e5/subraces")]
    subraces: HashMap<String, subrace::Subrace>,
}
