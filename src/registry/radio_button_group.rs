use super::types::RegistryEntry;
use crate::demos::demo_radio_button_group::DemoRadioButtonGroup;
use crate::demos::demo_radio_button_group_rtl::DemoRadioButtonGroupRtl;
use crate::markdown::converter::MdComponents;
use dioxus::prelude::*;

pub static RADIO_BUTTON_GROUP: RegistryEntry = RegistryEntry {
    slug: "radio-button-group",
    raw: include_str!("../../public/docs/radio_button_group.md"),
    tags: &[],
    components: radio_button_group_components,
};

fn radio_button_group_components() -> MdComponents {
    let mut c = MdComponents::new();
    c.add("DemoRadioButtonGroup", |_| rsx! { DemoRadioButtonGroup {} });
    c.add("DemoRadioButtonGroupRtl", |_| rsx! { DemoRadioButtonGroupRtl {} });
    c
}
