use dioxus::prelude::*;

use crate::ui::select_native::{LabelNative, SelectNative};

const TARGET_ID: &str = "select-native-error";

#[component]
pub fn DemoSelectNativeError() -> Element {
    rsx! {
        div { class: "space-y-2 [&>_svg]:text-destructive/80 min-w-[300px]",
            LabelNative { html_for: TARGET_ID, "Select with error (native)" }
            div { class: "border-destructive/80 text-destructive [&_select]:border-destructive/80 [&_select]:text-destructive [&_select:focus-visible]:border-destructive/80 [&_select:focus-visible]:ring-destructive/20",
                SelectNative { id: TARGET_ID,
                    option { value: "1", "React" }
                    option { value: "2", "Next.js" }
                    option { value: "3", "Astro" }
                    option { value: "4", "Gatsby" }
                }
            }
            p { class: "mt-2 text-xs text-destructive", role: "alert", "aria-live": "polite",
                "Selected option is invalid"
            }
        }
    }
}
