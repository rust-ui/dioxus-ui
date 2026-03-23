use dioxus::prelude::*;

use crate::ui::toggle_group::{ToggleGroup, ToggleGroupItem};

#[component]
pub fn DemoToggleGroup() -> Element {
    let mut selected = use_signal(|| "center".to_string());

    rsx! {
        div { class: "flex flex-col gap-6 items-center",
            // Text alignment group
            ToggleGroup {
                ToggleGroupItem {
                    value: "left",
                    pressed: selected() == "left",
                    onclick: move |_| selected.set("left".to_string()),
                    svg {
                        xmlns: "http://www.w3.org/2000/svg", class: "size-4",
                        view_box: "0 0 24 24", fill: "none", stroke: "currentColor",
                        stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                        line { x1: "21", x2: "3", y1: "6", y2: "6" }
                        line { x1: "15", x2: "3", y1: "12", y2: "12" }
                        line { x1: "17", x2: "3", y1: "18", y2: "18" }
                    }
                }
                ToggleGroupItem {
                    value: "center",
                    pressed: selected() == "center",
                    onclick: move |_| selected.set("center".to_string()),
                    svg {
                        xmlns: "http://www.w3.org/2000/svg", class: "size-4",
                        view_box: "0 0 24 24", fill: "none", stroke: "currentColor",
                        stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                        line { x1: "21", x2: "3", y1: "6", y2: "6" }
                        line { x1: "18", x2: "6", y1: "12", y2: "12" }
                        line { x1: "21", x2: "3", y1: "18", y2: "18" }
                    }
                }
                ToggleGroupItem {
                    value: "right",
                    pressed: selected() == "right",
                    onclick: move |_| selected.set("right".to_string()),
                    svg {
                        xmlns: "http://www.w3.org/2000/svg", class: "size-4",
                        view_box: "0 0 24 24", fill: "none", stroke: "currentColor",
                        stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                        line { x1: "21", x2: "3", y1: "6", y2: "6" }
                        line { x1: "21", x2: "9", y1: "12", y2: "12" }
                        line { x1: "21", x2: "7", y1: "18", y2: "18" }
                    }
                }
            }
        }
    }
}
