use dioxus::prelude::*;

use crate::ui::slider::Slider;

#[component]
pub fn DemoSliderDisabled() -> Element {
    rsx! {
        div { class: "flex flex-col gap-6 w-64",
            Slider { value: 40.0 }
            Slider { value: 25.0, step: 5.0 }
            Slider { value: 64.0, disabled: true }
        }
    }
}
