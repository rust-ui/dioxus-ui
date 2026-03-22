use dioxus::prelude::*;

use crate::demos::demo_input::DemoInput;
use crate::demos::demo_input_copy::DemoInputCopy;
use crate::markdown::converter::{MdComponents, MdNodeProps};
use crate::registry::RegistryEntry;

pub static INPUT: RegistryEntry = RegistryEntry {
    slug: "input",
    raw: include_str!("../../public/docs/input.md"),
    tags: &["interactive", "form"],
    components: input_components,
};

fn input_components() -> MdComponents {
    let mut c = MdComponents::new();
    c.add("DemoInput", |_: MdNodeProps| rsx! { DemoInput {} });
    c.add("DemoInputCopy", |_: MdNodeProps| rsx! { DemoInputCopy {} });
    c
}
