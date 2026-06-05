use dioxus::prelude::*;
use registry::demos::demo_hover_card::DemoHoverCard;
use registry::demos::demo_hover_card_rtl::DemoHoverCardRtl;

use super::types::RegistryEntry;
use crate::markdown::converter::MdComponents;

pub static HOVER_CARD: RegistryEntry = RegistryEntry {
    slug: "hover-card",
    raw: include_str!("../../public/docs/hover_card.md"),
    tags: &[],
    components: hover_card_components,
};

fn hover_card_components() -> MdComponents {
    let mut c = MdComponents::new();
    c.add("DemoHoverCard", |_| rsx! { DemoHoverCard {} });
    c.add("DemoHoverCardRtl", |_| rsx! { DemoHoverCardRtl {} });
    c
}
