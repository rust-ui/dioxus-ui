use dioxus::prelude::*;

use crate::demos::demo_checkbox::DemoCheckbox;
use crate::demos::demo_checkbox_disabled::DemoCheckboxDisabled;
use crate::demos::demo_checkbox_labeled::DemoCheckboxLabeled;
use crate::markdown::converter::{MdComponents, MdNodeProps};
use crate::registry::RegistryEntry;

pub static CHECKBOX: RegistryEntry = RegistryEntry {
    slug: "checkbox",
    raw: include_str!("../../public/docs/checkbox.md"),
    tags: &["interactive", "form"],
    components: checkbox_components,
};

fn checkbox_components() -> MdComponents {
    let mut c = MdComponents::new();
    c.add("DemoCheckbox", |_: MdNodeProps| rsx! { DemoCheckbox {} });
    c.add("DemoCheckboxLabeled", |_: MdNodeProps| rsx! { DemoCheckboxLabeled {} });
    c.add("DemoCheckboxDisabled", |_: MdNodeProps| rsx! { DemoCheckboxDisabled {} });
    c
}
