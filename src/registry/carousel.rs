use dioxus::prelude::*;
use registry::demos::demo_carousel::DemoCarousel;

use super::types::RegistryEntry;
use crate::markdown::converter::MdComponents;

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
