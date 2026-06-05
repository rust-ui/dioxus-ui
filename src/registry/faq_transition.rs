use dioxus::prelude::*;
use registry::demos::demo_faq_transition::DemoFaqTransition;

use super::types::RegistryEntry;
use crate::markdown::converter::MdComponents;

pub static FAQ_TRANSITION: RegistryEntry = RegistryEntry {
    slug: "faq-transition",
    raw: include_str!("../../public/docs/faq_transition.md"),
    tags: &[],
    components: faq_transition_components,
};

fn faq_transition_components() -> MdComponents {
    let mut c = MdComponents::new();
    c.add("DemoFaqTransition", |_| rsx! { DemoFaqTransition {} });
    c
}
