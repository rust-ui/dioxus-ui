use dioxus::prelude::*;

use crate::demos::demo_slider::DemoSlider;
use crate::markdown::converter::{MdComponents, MdNodeProps};
use crate::registry::RegistryEntry;

pub static SLIDER: RegistryEntry = RegistryEntry {
    slug: "slider",
    raw: include_str!("../../public/docs/slider.md"),
    tags: &["form"],
    components: slider_components,
};

fn slider_components() -> MdComponents {
    let mut c = MdComponents::new();
    c.add("DemoSlider", |_: MdNodeProps| rsx! { DemoSlider {} });
    c
}
