use dioxus::prelude::*;

use crate::ui::avatar::{Avatar, AvatarFallback, AvatarSize};

#[component]
pub fn DemoAvatarSizes() -> Element {
    rsx! {
        div { class: "flex items-center gap-4",
            Avatar { size: AvatarSize::Sm, AvatarFallback { "SM" } }
            Avatar { AvatarFallback { "MD" } }
            Avatar { size: AvatarSize::Lg, AvatarFallback { "LG" } }
        }
    }
}
