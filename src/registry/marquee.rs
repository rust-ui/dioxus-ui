use dioxus::prelude::*;
use registry::demos::demo_marquee::DemoMarquee;

use super::types::RegistryEntry;
use crate::markdown::converter::MdComponents;

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
