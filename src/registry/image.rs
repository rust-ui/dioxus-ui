use super::types::RegistryEntry;
use crate::demos::demo_image::DemoImage;
use crate::markdown::converter::MdComponents;
use dioxus::prelude::*;

pub static IMAGE: RegistryEntry = RegistryEntry {
    slug: "image",
    raw: include_str!("../../public/docs/image.md"),
    tags: &[],
    components: image_components,
};

fn image_components() -> MdComponents {
    let mut c = MdComponents::new();
    c.add("DemoImage", |_| rsx! { DemoImage {} });
    c
}
