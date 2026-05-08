use super::types::RegistryEntry;
use crate::demos::demo_aspect_ratio::DemoAspectRatio;
use crate::markdown::converter::MdComponents;
use dioxus::prelude::*;

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
