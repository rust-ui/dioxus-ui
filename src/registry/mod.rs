pub mod alert;
pub mod avatar;
pub mod badge;
pub mod button;
pub mod card;
pub mod checkbox;
pub mod input;
pub mod kbd;
pub mod label;
pub mod progress;
pub mod separator;
pub mod skeleton;
pub mod spinner;
pub mod switch;
pub mod tabs;
pub mod textarea;

use crate::markdown::converter::MdComponents;
use crate::markdown::parse_md;

use alert::ALERT;
use avatar::AVATAR;
use badge::BADGE;
use button::BUTTON;
use card::CARD;
use checkbox::CHECKBOX;
use input::INPUT;
use kbd::KBD;
use label::LABEL;
use progress::PROGRESS;
use separator::SEPARATOR;
use skeleton::SKELETON;
use spinner::SPINNER;
use switch::SWITCH;
use tabs::TABS;
use textarea::TEXTAREA;

pub struct RegistryEntry {
    pub slug: &'static str,
    pub raw: &'static str,
    pub tags: &'static [&'static str],
    pub components: fn() -> MdComponents,
}

impl RegistryEntry {
    pub fn title(&self) -> String {
        parse_md(self.raw).0.title
    }

    pub fn description(&self) -> String {
        parse_md(self.raw).0.description
    }

    pub fn body_md(&self) -> &str {
        parse_md(self.raw).1
    }
}

pub static REGISTRY: &[&RegistryEntry] = &[
    &ALERT,
    &AVATAR,
    &BADGE,
    &BUTTON,
    &CARD,
    &CHECKBOX,
    &INPUT,
    &KBD,
    &LABEL,
    &PROGRESS,
    &SEPARATOR,
    &SKELETON,
    &SPINNER,
    &SWITCH,
    &TABS,
    &TEXTAREA,
];

pub fn find(slug: &str) -> Option<&'static RegistryEntry> {
    REGISTRY.iter().copied().find(|e| e.slug == slug)
}

pub fn prev_next(slug: &str) -> (Option<&'static str>, Option<&'static str>) {
    let pos = REGISTRY.iter().position(|e| e.slug == slug);
    match pos {
        None => (None, None),
        Some(i) => (
            if i > 0 { Some(REGISTRY[i - 1].slug) } else { None },
            if i + 1 < REGISTRY.len() { Some(REGISTRY[i + 1].slug) } else { None },
        ),
    }
}
