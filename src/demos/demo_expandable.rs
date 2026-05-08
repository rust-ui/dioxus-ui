use dioxus::prelude::*;
use icons::Settings;

use crate::ui::expandable::{ExpandableContent, ExpandableTrigger, ExpandableTransition};

#[component]
pub fn DemoExpandable() -> Element {
    rsx! {
        div { class: "flex justify-center items-center min-h-[120px]",
            ExpandableTrigger {
                ExpandableTransition {
                    div { class: "flex items-center gap-2 px-3",
                        Settings { class: "size-4" }
                        span { class: "text-sm font-medium", "Settings" }
                    }
                }
                ExpandableContent {
                    div { class: "p-4",
                        p { class: "text-sm font-medium mb-2", "Quick Settings" }
                        div { class: "flex flex-col gap-2",
                            p { class: "text-sm text-muted-foreground", "Theme" }
                            p { class: "text-sm text-muted-foreground", "Language" }
                            p { class: "text-sm text-muted-foreground", "Notifications" }
                        }
                    }
                }
            }
        }
    }
}
