use dioxus::prelude::*;

use crate::demos::demo_textarea::DemoTextarea;
use crate::demos::demo_textarea_disabled::DemoTextareaDisabled;
use crate::markdown::converter::{MdComponents, MdNodeProps};
use crate::registry::RegistryEntry;

pub static TEXTAREA: RegistryEntry = RegistryEntry {
    slug: "textarea",
    raw: include_str!("../../public/docs/textarea.md"),
    tags: &["form", "input"],
    components: textarea_components,
};

fn textarea_components() -> MdComponents {
    let mut c = MdComponents::new();
    c.add("DemoTextarea", |_: MdNodeProps| rsx! { DemoTextarea {} });
    c.add("DemoTextareaDisabled", |_: MdNodeProps| rsx! { DemoTextareaDisabled {} });
    c
}
