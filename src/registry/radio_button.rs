use dioxus::prelude::*;
use registry::demos::demo_radio_button::DemoRadioButton;

use super::types::RegistryEntry;
use crate::markdown::converter::MdComponents;

pub static RADIO_BUTTON: RegistryEntry = RegistryEntry {
    slug: "radio-button",
    raw: include_str!("../../public/docs/radio_button.md"),
    tags: &[],
    components: radio_button_components,
};

fn radio_button_components() -> MdComponents {
    let mut c = MdComponents::new();
    c.add("DemoRadioButton", |_| rsx! { DemoRadioButton {} });
    c
}
