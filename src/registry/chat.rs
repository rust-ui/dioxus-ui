use super::RegistryEntry;
use crate::demos::demo_chat::DemoChat;
use crate::markdown::converter::MdComponents;
use dioxus::prelude::*;

pub static CHAT: RegistryEntry = RegistryEntry {
    slug: "chat",
    raw: include_str!("../../public/docs/chat.md"),
    tags: &[],
    components: chat_components,
};

fn chat_components() -> MdComponents {
    let mut c = MdComponents::new();
    c.add("DemoChat", |_| rsx! { DemoChat {} });
    c
}
