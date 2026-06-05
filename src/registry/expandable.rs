use dioxus::prelude::*;
use registry::demos::demo_expandable::DemoExpandable;

use super::types::RegistryEntry;
use crate::markdown::converter::MdComponents;

pub static EXPANDABLE: RegistryEntry = RegistryEntry {
    slug: "expandable",
    raw: include_str!("../../public/docs/expandable.md"),
    tags: &[],
    components: expandable_components,
};

fn expandable_components() -> MdComponents {
    let mut c = MdComponents::new();
    c.add("DemoExpandable", |_| rsx! { DemoExpandable {} });
    c
}
