use dioxus::prelude::*;

use crate::demos::demo_empty::DemoEmpty;
use crate::demos::demo_empty_muted::DemoEmptyMuted;
use crate::markdown::converter::{MdComponents, MdNodeProps};
use crate::registry::RegistryEntry;

pub static EMPTY: RegistryEntry = RegistryEntry {
    slug: "empty",
    raw: include_str!("../../public/docs/empty.md"),
    tags: &["display"],
    components: empty_components,
};

fn empty_components() -> MdComponents {
    let mut c = MdComponents::new();
    c.add("DemoEmpty", |_: MdNodeProps| rsx! { DemoEmpty {} });
    c.add("DemoEmptyMuted", |_: MdNodeProps| rsx! { DemoEmptyMuted {} });
    c
}
