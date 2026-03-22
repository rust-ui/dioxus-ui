use dioxus::prelude::*;

use crate::demos::demo_tabs::DemoTabs;
use crate::markdown::converter::{MdComponents, MdNodeProps};
use crate::registry::RegistryEntry;

pub static TABS: RegistryEntry = RegistryEntry {
    slug: "tabs",
    raw: include_str!("../../public/docs/tabs.md"),
    tags: &["navigation", "layout"],
    components: tabs_components,
};

fn tabs_components() -> MdComponents {
    let mut c = MdComponents::new();
    c.add("DemoTabs", |_: MdNodeProps| rsx! { DemoTabs {} });
    c
}
