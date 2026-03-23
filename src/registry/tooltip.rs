use dioxus::prelude::*;

use crate::demos::demo_tooltip::DemoTooltip;
use crate::markdown::converter::{MdComponents, MdNodeProps};
use crate::registry::RegistryEntry;

pub static TOOLTIP: RegistryEntry = RegistryEntry {
    slug: "tooltip",
    raw: include_str!("../../public/docs/tooltip.md"),
    tags: &["overlay"],
    components: tooltip_components,
};

fn tooltip_components() -> MdComponents {
    let mut c = MdComponents::new();
    c.add("DemoTooltip", |_: MdNodeProps| rsx! { DemoTooltip {} });
    c
}
