use dioxus::prelude::*;

use crate::ui::label::Label;
use crate::ui::select_native::SelectNative;

#[component]
pub fn DemoSelectNative() -> Element {
    let mut selected = use_signal(|| "apple".to_string());

    rsx! {
        div { class: "flex flex-col gap-2 w-48",
            Label { html_for: "fruit-select", "Pick a fruit" }
            SelectNative {
                id: "fruit-select",
                onchange: move |e: FormEvent| selected.set(e.value()),
                option { value: "apple", "Apple" }
                option { value: "banana", "Banana" }
                option { value: "cherry", "Cherry" }
                option { value: "date", "Date" }
                option { value: "elderberry", "Elderberry" }
            }
            p { class: "text-sm text-muted-foreground", "Selected: {selected}" }
        }
    }
}
