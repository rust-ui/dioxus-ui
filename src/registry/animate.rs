use super::RegistryEntry;
use crate::demos::demo_animate::DemoAnimate;
use crate::markdown::converter::MdComponents;
use dioxus::prelude::*;

pub static ANIMATE: RegistryEntry = RegistryEntry {
    slug: "animate",
    raw: include_str!("../../public/docs/animate.md"),
    tags: &[],
    components: animate_components,
};

fn animate_components() -> MdComponents {
    let mut c = MdComponents::new();
    c.add("DemoAnimate", |_| rsx! { DemoAnimate {} });
    c
}
