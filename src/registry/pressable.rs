use dioxus::prelude::*;
use registry::demos::demo_pressable::DemoPressable;

use super::types::RegistryEntry;
use crate::markdown::converter::MdComponents;

pub static PRESSABLE: RegistryEntry = RegistryEntry {
    slug: "pressable",
    raw: include_str!("../../public/docs/pressable.md"),
    tags: &[],
    components: pressable_components,
};

fn pressable_components() -> MdComponents {
    let mut c = MdComponents::new();
    c.add("DemoPressable", |_| rsx! { DemoPressable {} });
    c
}
