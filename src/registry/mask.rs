use dioxus::prelude::*;
use registry::demos::demo_mask::DemoMask;

use super::types::RegistryEntry;
use crate::markdown::converter::MdComponents;

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
