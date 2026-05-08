use super::types::RegistryEntry;
use crate::demos::demo_expandable::DemoExpandable;
use crate::markdown::converter::MdComponents;
use dioxus::prelude::*;

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
