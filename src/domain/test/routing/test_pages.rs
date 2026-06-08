use dioxus::prelude::*;

use crate::domain::test::components::demo_node_canvas::DemoNodeCanvas;
use crate::domain::test::components::demo_node_canvas_locked_mode::DemoNodeCanvasLockedMode;
use crate::domain::test::components::demo_node_canvas_multiselect::DemoNodeCanvasMultiselect;
use crate::domain::test::components::demo_node_canvas_minimap::DemoNodeCanvasMinimap;
use crate::domain::test::components::demo_node_canvas_status::DemoNodeCanvasStatus;
use crate::domain::test::components::demo_node_canvas_toolbar::DemoNodeCanvasToolbar;
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
                h2 { class: "text-xl font-semibold tracking-tight", "Node Canvas — Minimap" }
                p { class: "text-sm text-muted-foreground",
                    "10-node pipeline spread across canvas. Minimap shows full graph at a glance."
                }
            }
            DemoNodeCanvasMinimap {}

            div { class: "flex flex-col gap-1",
                h2 { class: "text-xl font-semibold tracking-tight", "Node Canvas — Toolbar" }
                p { class: "text-sm text-muted-foreground",
                    "Toolbar overlay on canvas. Add nodes, undo/redo, reset zoom, delete selected."
                }
            }
            DemoNodeCanvasToolbar {}

            div { class: "flex flex-col gap-1",
                h2 { class: "text-xl font-semibold tracking-tight", "Node Canvas — Locked Mode" }
                p { class: "text-sm text-muted-foreground",
                    "Read-only canvas. Toggle lock to freeze all interactions — no drag, pan, zoom, or connect."
                }
            }
            DemoNodeCanvasLockedMode {}

            div { class: "flex flex-col gap-1",
                h2 { class: "text-xl font-semibold tracking-tight", "Node Canvas — Multi-select" }
                p { class: "text-sm text-muted-foreground",
                    "Shift+click to add nodes to selection. Drag moves all selected. Del removes all."
                }
            }
            DemoNodeCanvasMultiselect {}

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
