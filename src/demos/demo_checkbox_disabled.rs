use dioxus::prelude::*;

use crate::ui::checkbox::Checkbox;
use crate::ui::label::Label;

#[component]
pub fn DemoCheckboxDisabled() -> Element {
    rsx! {
        div { class: "flex items-center gap-2 opacity-50",
            Checkbox { disabled: true }
            Label { "Disabled option" }
        }
    }
}
