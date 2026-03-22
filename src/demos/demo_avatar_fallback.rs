use dioxus::prelude::*;

use crate::ui::avatar::{Avatar, AvatarFallback};

#[component]
pub fn DemoAvatarFallback() -> Element {
    rsx! {
        Avatar {
            AvatarFallback { "JD" }
        }
    }
}
