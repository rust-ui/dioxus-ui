use dioxus::prelude::*;

use crate::ui::textarea::Textarea;

#[component]
pub fn DemoTextareaDisabled() -> Element {
    rsx! {
        Textarea { placeholder: "This textarea is disabled.", disabled: true }
    }
}
