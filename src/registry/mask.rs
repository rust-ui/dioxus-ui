use super::types::RegistryEntry;
use crate::demos::demo_mask::DemoMask;
use crate::markdown::converter::MdComponents;
use dioxus::prelude::*;

pub static MASK: RegistryEntry = RegistryEntry {
    slug: "mask",
    raw: include_str!("../../public/docs/mask.md"),
    tags: &[],
    components: mask_components,
};

fn mask_components() -> MdComponents {
    let mut c = MdComponents::new();
    c.add("DemoMask", |_| rsx! { DemoMask {} });
    c
}
