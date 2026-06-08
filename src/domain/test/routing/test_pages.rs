use dioxus::prelude::*;

use crate::domain::test::components::demo_node_canvas::DemoNodeCanvas;

#[component]
pub fn TestPage() -> Element {
    rsx! {
        div { class: "flex flex-col gap-6",
            div { class: "flex flex-col gap-1",
                h1 { class: "text-2xl font-semibold tracking-tight", "Node Canvas" }
                p { class: "text-sm text-muted-foreground", "Drag nodes. Bezier edges update live." }
            }
            DemoNodeCanvas {}
        }
    }
}
