use dioxus::prelude::*;

use crate::demos::demo_avatar::DemoAvatar;
use crate::demos::demo_avatar_fallback::DemoAvatarFallback;
use crate::demos::demo_avatar_sizes::DemoAvatarSizes;
use crate::markdown::converter::{MdComponents, MdNodeProps};
use crate::registry::RegistryEntry;

pub static AVATAR: RegistryEntry = RegistryEntry {
    slug: "avatar",
    raw: include_str!("../../public/docs/avatar.md"),
    tags: &["display", "user"],
    components: avatar_components,
};

fn avatar_components() -> MdComponents {
    let mut c = MdComponents::new();
    c.add("DemoAvatar", |_: MdNodeProps| rsx! { DemoAvatar {} });
    c.add("DemoAvatarFallback", |_: MdNodeProps| rsx! { DemoAvatarFallback {} });
    c.add("DemoAvatarSizes", |_: MdNodeProps| rsx! { DemoAvatarSizes {} });
    c
}
