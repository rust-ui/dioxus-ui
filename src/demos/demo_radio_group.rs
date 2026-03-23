use dioxus::prelude::*;

use crate::ui::radio_group::{RadioGroup, RadioItem};

#[component]
pub fn DemoRadioGroup() -> Element {
    let mut selected = use_signal(|| "comfortable".to_string());

    rsx! {
        div { class: "flex flex-col gap-4",
            RadioGroup {
                RadioItem {
                    name: "spacing",
                    value: "default",
                    checked: selected() == "default",
                    onchange: move |_| selected.set("default".to_string()),
                    "Default"
                }
                RadioItem {
                    name: "spacing",
                    value: "comfortable",
                    checked: selected() == "comfortable",
                    onchange: move |_| selected.set("comfortable".to_string()),
                    "Comfortable"
                }
                RadioItem {
                    name: "spacing",
                    value: "compact",
                    checked: selected() == "compact",
                    onchange: move |_| selected.set("compact".to_string()),
                    "Compact"
                }
            }
            p { class: "text-sm text-muted-foreground", "Selected: {selected}" }
        }
    }
}
