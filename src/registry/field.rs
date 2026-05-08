use super::types::RegistryEntry;
use crate::demos::demo_field::DemoField;
use crate::demos::demo_field_rtl::DemoFieldRtl;
use crate::markdown::converter::MdComponents;
use dioxus::prelude::*;

pub static FIELD: RegistryEntry = RegistryEntry {
    slug: "field",
    raw: include_str!("../../public/docs/field.md"),
    tags: &[],
    components: field_components,
};

fn field_components() -> MdComponents {
    let mut c = MdComponents::new();
    c.add("DemoField", |_| rsx! { DemoField {} });
    c.add("DemoFieldRtl", |_| rsx! { DemoFieldRtl {} });
    c
}
