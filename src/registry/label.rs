use dioxus::prelude::*;

use crate::demos::demo_label::DemoLabel;
use crate::demos::demo_label_input::DemoLabelInput;
use crate::markdown::converter::{MdComponents, MdNodeProps};
use crate::registry::RegistryEntry;

pub static LABEL: RegistryEntry = RegistryEntry {
    slug: "label",
    raw: include_str!("../../public/docs/label.md"),
    tags: &["form", "accessibility"],
    components: label_components,
};

fn label_components() -> MdComponents {
    let mut c = MdComponents::new();
    c.add("DemoLabel", |_: MdNodeProps| rsx! { DemoLabel {} });
    c.add("DemoLabelInput", |_: MdNodeProps| rsx! { DemoLabelInput {} });
    c
}
