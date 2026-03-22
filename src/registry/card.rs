use dioxus::prelude::*;

use crate::demos::demo_card::DemoCard;
use crate::demos::demo_card_action::DemoCardAction;
use crate::demos::demo_card_group::DemoCardGroup;
use crate::demos::demo_card_reverse::DemoCardReverse;
use crate::demos::demo_card_sm::DemoCardSm;
use crate::markdown::converter::{MdComponents, MdNodeProps};
use crate::registry::RegistryEntry;

pub static CARD: RegistryEntry = RegistryEntry {
    slug: "card",
    raw: include_str!("../../public/docs/card.md"),
    components: card_components,
};

fn card_components() -> MdComponents {
    let mut c = MdComponents::new();
    c.add("demo-card", |_: MdNodeProps| rsx! { DemoCard {} });
    c.add("demo-card-action", |_: MdNodeProps| rsx! { DemoCardAction {} });
    c.add("demo-card-group", |_: MdNodeProps| rsx! { DemoCardGroup {} });
    c.add("demo-card-sm", |_: MdNodeProps| rsx! { DemoCardSm {} });
    c.add("demo-card-reverse", |_: MdNodeProps| rsx! { DemoCardReverse {} });
    c
}
