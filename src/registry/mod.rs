pub mod badge;
pub mod button;
pub mod card;
pub mod input;
pub mod separator;
pub mod skeleton;
pub mod spinner;

use crate::markdown::converter::MdComponents;
use crate::markdown::parse_md;

use badge::BADGE;
use button::BUTTON;
use card::CARD;
use input::INPUT;
use separator::SEPARATOR;
use skeleton::SKELETON;
use spinner::SPINNER;

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
    &BUTTON, &BADGE, &CARD, &INPUT, &SEPARATOR, &SKELETON, &SPINNER,
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
