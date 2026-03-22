use dioxus::prelude::*;

use crate::demos::demo_kbd::DemoKbd;
use crate::demos::demo_kbd_combination::DemoKbdCombination;
use crate::markdown::converter::{MdComponents, MdNodeProps};
use crate::registry::RegistryEntry;

pub static KBD: RegistryEntry = RegistryEntry {
    slug: "kbd",
    raw: include_str!("../../public/docs/kbd.md"),
    tags: &["display", "typography"],
    components: kbd_components,
};

fn kbd_components() -> MdComponents {
    let mut c = MdComponents::new();
    c.add("DemoKbd", |_: MdNodeProps| rsx! { DemoKbd {} });
    c.add("DemoKbdCombination", |_: MdNodeProps| rsx! { DemoKbdCombination {} });
    c
}
