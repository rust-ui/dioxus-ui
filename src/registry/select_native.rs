use dioxus::prelude::*;

use crate::demos::demo_select_native::DemoSelectNative;
use crate::demos::demo_select_native_group::DemoSelectNativeGroup;
use crate::markdown::converter::{MdComponents, MdNodeProps};
use crate::registry::RegistryEntry;

pub static SELECT_NATIVE: RegistryEntry = RegistryEntry {
    slug: "select-native",
    raw: include_str!("../../public/docs/select_native.md"),
    tags: &["form"],
    components: select_native_components,
};

fn select_native_components() -> MdComponents {
    let mut c = MdComponents::new();
    c.add("DemoSelectNative", |_: MdNodeProps| rsx! { DemoSelectNative {} });
    c.add("DemoSelectNativeGroup", |_: MdNodeProps| rsx! { DemoSelectNativeGroup {} });
    c
}
