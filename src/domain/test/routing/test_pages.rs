use dioxus::prelude::*;

use crate::domain::test::components::demo_node_canvas::DemoNodeCanvas;
use crate::domain::test::components::demo_node_canvas_status::DemoNodeCanvasStatus;
use crate::domain::test::components::demo_toolbar::DemoToolbar;

#[component]
pub fn TestPage() -> Element {
    rsx! {
        div { class: "flex flex-col gap-6",
            div { class: "flex flex-col gap-1",
                h1 { class: "text-2xl font-semibold tracking-tight", "Node Canvas" }
                p { class: "text-sm text-muted-foreground", "Drag nodes. Bezier edges update live." }
            }
            DemoNodeCanvas {}

            div { class: "flex flex-col gap-1",
                h2 { class: "text-xl font-semibold tracking-tight", "Node Canvas — Status" }
                p { class: "text-sm text-muted-foreground", "Node execution status badges. Hit Run / Error / Reset." }
            }
            DemoNodeCanvasStatus {}

            div { class: "flex flex-col gap-1",
                h2 { class: "text-xl font-semibold tracking-tight", "Toolbar" }
                p { class: "text-sm text-muted-foreground",
                    "Composable toolbar with toggle groups, buttons, separators, and links."
                }
            }
            DemoToolbar {}
        }
    }
}
