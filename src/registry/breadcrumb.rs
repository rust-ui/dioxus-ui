use dioxus::prelude::*;

use crate::demos::demo_breadcrumb::DemoBreadcrumb;
use crate::markdown::converter::{MdComponents, MdNodeProps};
use crate::registry::RegistryEntry;

pub static BREADCRUMB: RegistryEntry = RegistryEntry {
    slug: "breadcrumb",
    raw: include_str!("../../public/docs/breadcrumb.md"),
    tags: &["navigation"],
    components: breadcrumb_components,
};

fn breadcrumb_components() -> MdComponents {
    let mut c = MdComponents::new();
    c.add("DemoBreadcrumb", |_: MdNodeProps| rsx! { DemoBreadcrumb {} });
    c
}
