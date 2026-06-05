use dioxus::prelude::*;
use registry::demos::demo_shimmer::DemoShimmer;

use super::types::RegistryEntry;
use crate::markdown::converter::MdComponents;

pub static SHIMMER: RegistryEntry = RegistryEntry {
    slug: "shimmer",
    raw: include_str!("../../public/docs/shimmer.md"),
    tags: &[],
    components: shimmer_components,
};

fn shimmer_components() -> MdComponents {
    let mut c = MdComponents::new();
    c.add("DemoShimmer", |_| rsx! { DemoShimmer {} });
    c
}
