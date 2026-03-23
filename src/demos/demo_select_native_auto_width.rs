use dioxus::prelude::*;

use crate::ui::select_native::{LabelNative, SelectNative};

const TARGET_ID: &str = "select-native-auto-width";

#[component]
pub fn DemoSelectNativeAutoWidth() -> Element {
    rsx! {
        div { class: "space-y-2 min-w-[300px]",
            LabelNative { html_for: TARGET_ID, "Select with auto-width (native)" }
            div { class: "w-fit",
                SelectNative { id: TARGET_ID,
                    option { value: "1", "React" }
                    option { value: "2", "Next.js" }
                    option { value: "3", "Astro" }
                    option { value: "4", "Gatsby" }
                }
            }
        }
    }
}
