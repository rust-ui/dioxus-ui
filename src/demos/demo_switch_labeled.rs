use dioxus::prelude::*;

use crate::ui::label::Label;
use crate::ui::switch::Switch;

#[component]
pub fn DemoSwitchLabeled() -> Element {
    rsx! {
        div { class: "flex items-center gap-2",
            Switch {}
            Label { "Airplane mode" }
        }
    }
}
