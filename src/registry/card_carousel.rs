use super::types::RegistryEntry;
use crate::demos::demo_card_carousel::DemoCardCarousel;
use crate::markdown::converter::MdComponents;
use dioxus::prelude::*;

pub static CARD_CAROUSEL: RegistryEntry = RegistryEntry {
    slug: "card-carousel",
    raw: include_str!("../../public/docs/card_carousel.md"),
    tags: &[],
    components: card_carousel_components,
};

fn card_carousel_components() -> MdComponents {
    let mut c = MdComponents::new();
    c.add("DemoCardCarousel", |_| rsx! { DemoCardCarousel {} });
    c
}
