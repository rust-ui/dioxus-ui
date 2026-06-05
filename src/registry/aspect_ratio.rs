use dioxus::prelude::*;
use registry::demos::demo_aspect_ratio::DemoAspectRatio;

use super::types::RegistryEntry;
use crate::markdown::converter::MdComponents;

pub static ASPECT_RATIO: RegistryEntry = RegistryEntry {
    slug: "aspect-ratio",
    raw: include_str!("../../public/docs/aspect_ratio.md"),
    tags: &[],
    components: aspect_ratio_components,
};

fn aspect_ratio_components() -> MdComponents {
    let mut c = MdComponents::new();
    c.add("DemoAspectRatio", |_| rsx! { DemoAspectRatio {} });
    c
}
