use dioxus::prelude::*;

use crate::demos::demo_pagination::DemoPagination;
use crate::markdown::converter::{MdComponents, MdNodeProps};
use crate::registry::RegistryEntry;

pub static PAGINATION: RegistryEntry = RegistryEntry {
    slug: "pagination",
    raw: include_str!("../../public/docs/pagination.md"),
    tags: &["navigation"],
    components: pagination_components,
};

fn pagination_components() -> MdComponents {
    let mut c = MdComponents::new();
    c.add("DemoPagination", |_: MdNodeProps| rsx! { DemoPagination {} });
    c
}
