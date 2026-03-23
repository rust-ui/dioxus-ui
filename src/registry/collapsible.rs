use dioxus::prelude::*;

use crate::demos::demo_collapsible::DemoCollapsible;
use crate::markdown::converter::{MdComponents, MdNodeProps};
use crate::registry::RegistryEntry;

pub static COLLAPSIBLE: RegistryEntry = RegistryEntry {
    slug: "collapsible",
    raw: include_str!("../../public/docs/collapsible.md"),
    tags: &["interactive"],
    components: collapsible_components,
};

fn collapsible_components() -> MdComponents {
    let mut c = MdComponents::new();
    c.add("DemoCollapsible", |_: MdNodeProps| rsx! { DemoCollapsible {} });
    c
}
