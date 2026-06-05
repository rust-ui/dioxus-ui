use dioxus::prelude::*;
use registry::demos::demo_popover::DemoPopover;

use super::RegistryEntry;
use crate::markdown::converter::MdComponents;

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
