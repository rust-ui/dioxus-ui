use super::types::RegistryEntry;
use crate::demos::demo_marquee::DemoMarquee;
use crate::markdown::converter::MdComponents;
use dioxus::prelude::*;

pub static MARQUEE: RegistryEntry = RegistryEntry {
    slug: "marquee",
    raw: include_str!("../../public/docs/marquee.md"),
    tags: &[],
    components: marquee_components,
};

fn marquee_components() -> MdComponents {
    let mut c = MdComponents::new();
    c.add("DemoMarquee", |_| rsx! { DemoMarquee {} });
    c
}
