use dioxus::prelude::*;

use crate::ui::label::Label;
use crate::ui::select_native::SelectNative;

#[component]
pub fn DemoSelectNativeGroup() -> Element {
    rsx! {
        div { class: "flex flex-col gap-2 w-72",
            Label { html_for: "select-group", "Select with option groups" }
            SelectNative { id: "select-group",
                optgroup { label: "Frontend",
                    option { value: "react", "React" }
                    option { value: "vue", "Vue" }
                    option { value: "angular", "Angular" }
                }
                optgroup { label: "Backend",
                    option { value: "leptos", "Leptos" }
                    option { value: "axum", "Axum" }
                    option { value: "actix", "Actix" }
                }
            }
        }
    }
}
