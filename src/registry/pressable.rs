use super::types::RegistryEntry;
use crate::demos::demo_pressable::DemoPressable;
use crate::markdown::converter::MdComponents;
use dioxus::prelude::*;

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
