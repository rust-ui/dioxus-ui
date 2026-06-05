use dioxus::prelude::*;
use registry::demos::demo_bottom_nav::DemoBottomNav;

use super::types::RegistryEntry;
use crate::markdown::converter::MdComponents;

pub static BOTTOM_NAV: RegistryEntry = RegistryEntry {
    slug: "bottom-nav",
    raw: include_str!("../../public/docs/bottom_nav.md"),
    tags: &[],
    components: bottom_nav_components,
};

fn bottom_nav_components() -> MdComponents {
    let mut c = MdComponents::new();
    c.add("DemoBottomNav", |_| rsx! { DemoBottomNav {} });
    c
}
