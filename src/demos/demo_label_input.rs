use dioxus::prelude::*;

use crate::ui::input::Input;
use crate::ui::label::Label;

#[component]
pub fn DemoLabelInput() -> Element {
    rsx! {
        div { class: "flex flex-col gap-2",
            Label { html_for: "username", "Username" }
            Input { id: "username", placeholder: "Enter your username" }
        }
    }
}
