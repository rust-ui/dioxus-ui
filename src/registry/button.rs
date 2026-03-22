use dioxus::prelude::*;

use crate::demos::demo_button::DemoButton;
use crate::demos::demo_button_variants::DemoButtonVariants;
use crate::markdown::converter::{MdComponents, MdNodeProps};
use crate::registry::RegistryEntry;

pub static BUTTON: RegistryEntry = RegistryEntry {
    slug: "button",
    raw: include_str!("../../public/docs/button.md"),
    components: button_components,
};

fn button_components() -> MdComponents {
    let mut c = MdComponents::new();
    c.add("demo-button", |_: MdNodeProps| rsx! { DemoButton {} });
    c.add("demo-button-variants", |_: MdNodeProps| rsx! { DemoButtonVariants {} });
    c
}
