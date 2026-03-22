use dioxus::prelude::*;

use crate::demos::demo_switch::DemoSwitch;
use crate::demos::demo_switch_labeled::DemoSwitchLabeled;
use crate::markdown::converter::{MdComponents, MdNodeProps};
use crate::registry::RegistryEntry;

pub static SWITCH: RegistryEntry = RegistryEntry {
    slug: "switch",
    raw: include_str!("../../public/docs/switch.md"),
    tags: &["interactive", "form"],
    components: switch_components,
};

fn switch_components() -> MdComponents {
    let mut c = MdComponents::new();
    c.add("DemoSwitch", |_: MdNodeProps| rsx! { DemoSwitch {} });
    c.add("DemoSwitchLabeled", |_: MdNodeProps| rsx! { DemoSwitchLabeled {} });
    c
}
