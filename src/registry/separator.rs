use dioxus::prelude::*;

use crate::demos::demo_separator::DemoSeparator;
use crate::markdown::converter::{MdComponents, MdNodeProps};
use crate::registry::RegistryEntry;

pub static SEPARATOR: RegistryEntry = RegistryEntry {
    slug: "separator",
    raw: include_str!("../../public/docs/separator.md"),
    tags: &["layout"],
    components: separator_components,
};

fn separator_components() -> MdComponents {
    let mut c = MdComponents::new();
    c.add("DemoSeparator", |_: MdNodeProps| rsx! { DemoSeparator {} });
    c
}
