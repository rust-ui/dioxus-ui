use super::RegistryEntry;
use crate::demos::demo_drawer::DemoDrawer;
use crate::markdown::converter::MdComponents;
use dioxus::prelude::*;

pub static DRAWER: RegistryEntry = RegistryEntry {
    slug: "drawer",
    raw: include_str!("../../public/docs/drawer.md"),
    tags: &[],
    components: drawer_components,
};

fn drawer_components() -> MdComponents {
    let mut c = MdComponents::new();
    c.add("DemoDrawer", |_| rsx! { DemoDrawer {} });
    c
}
