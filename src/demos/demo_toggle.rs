use dioxus::prelude::*;

use crate::ui::toggle::{Toggle, ToggleVariant};

#[component]
pub fn DemoToggle() -> Element {
    let mut pressed = use_signal(|| false);

    rsx! {
        div { class: "flex gap-4 items-center flex-wrap",
            Toggle {
                pressed: pressed(),
                onclick: move |_| pressed.toggle(),
                svg {
                    xmlns: "http://www.w3.org/2000/svg",
                    class: "size-4",
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    path { d: "M6 4h8a4 4 0 0 1 4 4 4 4 0 0 1-4 4H6z" }
                    path { d: "M6 12h9a4 4 0 0 1 4 4 4 4 0 0 1-4 4H6z" }
                }
                "Bold"
            }
            Toggle {
                variant: ToggleVariant::Outline,
                pressed: pressed(),
                onclick: move |_| pressed.toggle(),
                svg {
                    xmlns: "http://www.w3.org/2000/svg",
                    class: "size-4",
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    path { d: "M6 4h8a4 4 0 0 1 4 4 4 4 0 0 1-4 4H6z" }
                    path { d: "M6 12h9a4 4 0 0 1 4 4 4 4 0 0 1-4 4H6z" }
                }
                "Bold"
            }
        }
    }
}
