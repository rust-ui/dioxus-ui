use dioxus::prelude::*;

use crate::ui::select_native::{OverlappingLabel, SelectNative};

const TARGET_ID: &str = "select-native-overlapping-label";

#[component]
pub fn DemoSelectNativeOverlappingLabel() -> Element {
    rsx! {
        div { class: "relative group min-w-[300px]",
            OverlappingLabel { html_for: TARGET_ID, label: "Select with overlapping label (native)" }
            SelectNative { id: TARGET_ID,
                option { value: "", disabled: true, "Select framework" }
                option { value: "1", "React" }
                option { value: "2", "Next.js" }
                option { value: "3", "Astro" }
                option { value: "4", "Gatsby" }
            }
        }
    }
}
