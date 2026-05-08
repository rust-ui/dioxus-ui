use super::types::RegistryEntry;
use crate::demos::demo_item::DemoItem;
use crate::markdown::converter::MdComponents;
use dioxus::prelude::*;

pub static ITEM: RegistryEntry = RegistryEntry {
    slug: "item",
    raw: include_str!("../../public/docs/item.md"),
    tags: &[],
    components: item_components,
};

fn item_components() -> MdComponents {
    let mut c = MdComponents::new();
    c.add("DemoItem", |_| rsx! { DemoItem {} });
    c
}
