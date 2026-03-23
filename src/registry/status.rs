use dioxus::prelude::*;

use crate::demos::demo_status::DemoStatus;
use crate::demos::demo_status_variants::DemoStatusVariants;
use crate::markdown::converter::{MdComponents, MdNodeProps};
use crate::registry::RegistryEntry;

pub static STATUS: RegistryEntry = RegistryEntry {
    slug: "status",
    raw: include_str!("../../public/docs/status.md"),
    tags: &["display"],
    components: status_components,
};

fn status_components() -> MdComponents {
    let mut c = MdComponents::new();
    c.add("DemoStatus", |_: MdNodeProps| rsx! { DemoStatus {} });
    c.add("DemoStatusVariants", |_: MdNodeProps| rsx! { DemoStatusVariants {} });
    c
}
