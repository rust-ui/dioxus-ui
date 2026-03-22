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
    c.add("DemoCard", |_: MdNodeProps| rsx! { DemoCard {} });
    c.add("DemoCardAction", |_: MdNodeProps| rsx! { DemoCardAction {} });
    c.add("DemoCardGroup", |_: MdNodeProps| rsx! { DemoCardGroup {} });
    c.add("DemoCardSm", |_: MdNodeProps| rsx! { DemoCardSm {} });
    c.add("DemoCardReverse", |_: MdNodeProps| rsx! { DemoCardReverse {} });
    c
}
