use dioxus::prelude::*;

use crate::ui::badge::Badge;
use crate::ui::collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger};

#[component]
pub fn DemoCollapsible() -> Element {
    let mut open = use_signal(|| false);

    rsx! {
        div { class: "w-full max-w-sm",
            Collapsible {
                div { class: "flex items-center justify-between px-3 py-2",
                    div { class: "flex items-center gap-2",
                        Badge { "@dioxus-ui" }
                        span { class: "text-sm font-medium", "3 repositories" }
                    }
                    CollapsibleTrigger {
                        open: open,
                        onclick: move |_| open.toggle(),
                    }
                }
                div { class: "rounded-md border px-3 py-2 text-sm font-mono",
                    "dioxus-ui/button"
                }
                CollapsibleContent {
                    open: open,
                    div { class: "flex flex-col gap-1 mt-1",
                        div { class: "rounded-md border px-3 py-2 text-sm font-mono", "dioxus-ui/card" }
                        div { class: "rounded-md border px-3 py-2 text-sm font-mono", "dioxus-ui/input" }
                    }
                }
            }
        }
    }
}
