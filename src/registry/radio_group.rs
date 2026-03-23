use dioxus::prelude::*;

use crate::demos::demo_radio_group::DemoRadioGroup;
use crate::markdown::converter::{MdComponents, MdNodeProps};
use crate::registry::RegistryEntry;

pub static RADIO_GROUP: RegistryEntry = RegistryEntry {
    slug: "radio-group",
    raw: include_str!("../../public/docs/radio_group.md"),
    tags: &["form"],
    components: radio_group_components,
};

fn radio_group_components() -> MdComponents {
    let mut c = MdComponents::new();
    c.add("DemoRadioGroup", |_: MdNodeProps| rsx! { DemoRadioGroup {} });
    c
}
