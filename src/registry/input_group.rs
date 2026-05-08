use super::types::RegistryEntry;
use crate::demos::demo_input_group::DemoInputGroup;
use crate::demos::demo_input_group_block::DemoInputGroupBlock;
use crate::demos::demo_input_group_rtl::DemoInputGroupRtl;
use crate::demos::demo_input_group_text::DemoInputGroupText;
use crate::markdown::converter::MdComponents;
use dioxus::prelude::*;

pub static INPUT_GROUP: RegistryEntry = RegistryEntry {
    slug: "input-group",
    raw: include_str!("../../public/docs/input_group.md"),
    tags: &[],
    components: input_group_components,
};

fn input_group_components() -> MdComponents {
    let mut c = MdComponents::new();
    c.add("DemoInputGroup", |_| rsx! { DemoInputGroup {} });
    c.add("DemoInputGroupBlock", |_| rsx! { DemoInputGroupBlock {} });
    c.add("DemoInputGroupText", |_| rsx! { DemoInputGroupText {} });
    c.add("DemoInputGroupRtl", |_| rsx! { DemoInputGroupRtl {} });
    c
}
