use dioxus::prelude::*;

use crate::ui::kbd::Kbd;

#[component]
pub fn DemoKbdCombination() -> Element {
    rsx! {
        div { class: "flex items-center gap-1",
            Kbd { "Ctrl" }
            span { class: "text-muted-foreground text-xs", "+" }
            Kbd { "Shift" }
            span { class: "text-muted-foreground text-xs", "+" }
            Kbd { "P" }
        }
    }
}
