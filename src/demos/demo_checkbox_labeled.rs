use dioxus::prelude::*;

use crate::ui::checkbox::Checkbox;
use crate::ui::label::Label;

#[component]
pub fn DemoCheckboxLabeled() -> Element {
    rsx! {
        div { class: "flex items-center gap-2",
            Checkbox {}
            Label { "Accept terms and conditions" }
        }
    }
}
