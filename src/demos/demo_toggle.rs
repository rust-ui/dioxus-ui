use dioxus::prelude::*;

use crate::ui::toggle::Toggle;

#[component]
pub fn DemoToggle() -> Element {
    let mut pressed = use_signal(|| false);

    rsx! {
        Toggle {
            pressed: pressed(),
            onclick: move |_| pressed.set(!pressed()),
            "Bold"
        }
    }
}
