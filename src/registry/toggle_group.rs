use dioxus::prelude::*;

use crate::demos::demo_toggle_group::DemoToggleGroup;
use crate::markdown::converter::{MdComponents, MdNodeProps};
use crate::registry::RegistryEntry;

pub static TOGGLE_GROUP: RegistryEntry = RegistryEntry {
    slug: "toggle-group",
    raw: include_str!("../../public/docs/toggle_group.md"),
    tags: &["interactive"],
    components: toggle_group_components,
};

fn toggle_group_components() -> MdComponents {
    let mut c = MdComponents::new();
    c.add("DemoToggleGroup", |_: MdNodeProps| rsx! { DemoToggleGroup {} });
    c
}
