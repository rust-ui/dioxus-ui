use dioxus::prelude::*;

use crate::demos::demo_badge::DemoBadge;
use crate::demos::demo_badge_colors::DemoBadgeColors;
use crate::demos::demo_badge_custom::DemoBadgeCustom;
use crate::demos::demo_badge_variants::DemoBadgeVariants;
use crate::markdown::converter::{MdComponents, MdNodeProps};
use crate::registry::RegistryEntry;

pub static BADGE: RegistryEntry = RegistryEntry {
    slug: "badge",
    raw: include_str!("../../public/docs/badge.md"),
    components: badge_components,
};

fn badge_components() -> MdComponents {
    let mut c = MdComponents::new();
    c.add("DemoBadge", |_: MdNodeProps| rsx! { DemoBadge {} });
    c.add("DemoBadgeVariants", |_: MdNodeProps| rsx! { DemoBadgeVariants {} });
    c.add("DemoBadgeColors", |_: MdNodeProps| rsx! { DemoBadgeColors {} });
    c.add("DemoBadgeCustom", |_: MdNodeProps| rsx! { DemoBadgeCustom {} });
    c
}
