pub mod accordion;
pub mod alert;
pub mod alert_dialog;
pub mod avatar;
pub mod badge;
pub mod breadcrumb;
pub mod button;
pub mod card;
pub mod checkbox;
pub mod collapsible;
pub mod input;
pub mod kbd;
pub mod label;
pub mod progress;
pub mod radio_group;
pub mod select_native;
pub mod separator;
pub mod skeleton;
pub mod slider;
pub mod spinner;
pub mod switch;
pub mod table;
pub mod tabs;
pub mod textarea;
pub mod toggle;
pub mod tooltip;

use crate::markdown::converter::MdComponents;
use crate::markdown::parse_md;

use accordion::ACCORDION;
use alert::ALERT;
use alert_dialog::ALERT_DIALOG;
use avatar::AVATAR;
use badge::BADGE;
use breadcrumb::BREADCRUMB;
use button::BUTTON;
use card::CARD;
use checkbox::CHECKBOX;
use collapsible::COLLAPSIBLE;
use input::INPUT;
use kbd::KBD;
use label::LABEL;
use progress::PROGRESS;
use radio_group::RADIO_GROUP;
use select_native::SELECT_NATIVE;
use separator::SEPARATOR;
use skeleton::SKELETON;
use slider::SLIDER;
use spinner::SPINNER;
use switch::SWITCH;
use table::TABLE;
use tabs::TABS;
use textarea::TEXTAREA;
use toggle::TOGGLE;
use tooltip::TOOLTIP;

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
    &ACCORDION,
    &ALERT,
    &ALERT_DIALOG,
    &AVATAR,
    &BADGE,
    &BREADCRUMB,
    &BUTTON,
    &CARD,
    &CHECKBOX,
    &COLLAPSIBLE,
    &INPUT,
    &KBD,
    &LABEL,
    &PROGRESS,
    &RADIO_GROUP,
    &SELECT_NATIVE,
    &SEPARATOR,
    &SKELETON,
    &SLIDER,
    &SPINNER,
    &SWITCH,
    &TABLE,
    &TABS,
    &TEXTAREA,
    &TOGGLE,
    &TOOLTIP,
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
