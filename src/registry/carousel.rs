use super::types::RegistryEntry;
use crate::demos::demo_carousel::DemoCarousel;
use crate::markdown::converter::MdComponents;
use dioxus::prelude::*;

pub static CAROUSEL: RegistryEntry = RegistryEntry {
    slug: "carousel",
    raw: include_str!("../../public/docs/carousel.md"),
    tags: &[],
    components: carousel_components,
};

fn carousel_components() -> MdComponents {
    let mut c = MdComponents::new();
    c.add("DemoCarousel", |_| rsx! { DemoCarousel {} });
    c
}
