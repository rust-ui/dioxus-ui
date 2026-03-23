use dioxus::prelude::*;

use crate::demos::demo_table::DemoTable;
use crate::markdown::converter::{MdComponents, MdNodeProps};
use crate::registry::RegistryEntry;

pub static TABLE: RegistryEntry = RegistryEntry {
    slug: "table",
    raw: include_str!("../../public/docs/table.md"),
    tags: &["data"],
    components: table_components,
};

fn table_components() -> MdComponents {
    let mut c = MdComponents::new();
    c.add("DemoTable", |_: MdNodeProps| rsx! { DemoTable {} });
    c
}
