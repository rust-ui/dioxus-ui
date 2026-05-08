use super::types::RegistryEntry;
use crate::demos::demo_radio_button::DemoRadioButton;
use crate::markdown::converter::MdComponents;
use dioxus::prelude::*;

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
