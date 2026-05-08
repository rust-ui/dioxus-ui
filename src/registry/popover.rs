use super::RegistryEntry;
use crate::demos::demo_popover::DemoPopover;
use crate::markdown::converter::MdComponents;
use dioxus::prelude::*;

pub static POPOVER: RegistryEntry = RegistryEntry {
    slug: "popover",
    raw: include_str!("../../public/docs/popover.md"),
    tags: &[],
    components: popover_components,
};

fn popover_components() -> MdComponents {
    let mut c = MdComponents::new();
    c.add("DemoPopover", |_| rsx! { DemoPopover {} });
    c
}
