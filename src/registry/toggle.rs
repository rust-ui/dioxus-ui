use dioxus::prelude::*;

use crate::demos::demo_toggle::DemoToggle;
use crate::markdown::converter::{MdComponents, MdNodeProps};
use crate::registry::RegistryEntry;

pub static TOGGLE: RegistryEntry = RegistryEntry {
    slug: "toggle",
    raw: include_str!("../../public/docs/toggle.md"),
    tags: &["interactive"],
    components: toggle_components,
};

fn toggle_components() -> MdComponents {
    let mut c = MdComponents::new();
    c.add("DemoToggle", |_: MdNodeProps| rsx! { DemoToggle {} });
    c
}
