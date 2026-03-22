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
    c.add("demo-badge", |_: MdNodeProps| rsx! { DemoBadge {} });
    c.add("demo-badge-variants", |_: MdNodeProps| rsx! { DemoBadgeVariants {} });
    c.add("demo-badge-colors", |_: MdNodeProps| rsx! { DemoBadgeColors {} });
    c.add("demo-badge-custom", |_: MdNodeProps| rsx! { DemoBadgeCustom {} });
    c
}
