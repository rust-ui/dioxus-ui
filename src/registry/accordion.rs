use dioxus::prelude::*;

use crate::demos::demo_accordion::DemoAccordion;
use crate::demos::demo_accordion_bordered::DemoAccordionBordered;
use crate::markdown::converter::{MdComponents, MdNodeProps};
use crate::registry::RegistryEntry;

pub static ACCORDION: RegistryEntry = RegistryEntry {
    slug: "accordion",
    raw: include_str!("../../public/docs/accordion.md"),
    tags: &["interactive"],
    components: accordion_components,
};

fn accordion_components() -> MdComponents {
    let mut c = MdComponents::new();
    c.add("DemoAccordion", |_: MdNodeProps| rsx! { DemoAccordion {} });
    c.add("DemoAccordionBordered", |_: MdNodeProps| rsx! { DemoAccordionBordered {} });
    c
}
