use dioxus::prelude::*;

use crate::domain::test::demos::demo_toolbar::DemoToolbar;
use crate::domain::test::demos::demo_workflow::DemoWorkflow;
use crate::domain::test::demos::demo_workflow_copy_paste::DemoWorkflowCopyPaste;
use crate::domain::test::demos::demo_workflow_locked_mode::DemoWorkflowLockedMode;
use crate::domain::test::demos::demo_workflow_minimap::DemoWorkflowMinimap;
use crate::domain::test::demos::demo_workflow_multiselect::DemoWorkflowMultiselect;
use crate::domain::test::demos::demo_workflow_status::DemoWorkflowStatus;
use crate::domain::test::demos::demo_workflow_toolbar::DemoWorkflowToolbar;

#[component]
pub fn TestPage() -> Element {
    rsx! {
        div { class: "flex flex-col gap-6",
            div { class: "flex flex-col gap-1",
                h1 { class: "text-2xl font-semibold tracking-tight", "Workflow Canvas" }
                p { class: "text-sm text-muted-foreground", "Drag nodes. Bezier edges update live." }
            }
            DemoWorkflow {}

            div { class: "flex flex-col gap-1",
                h2 { class: "text-xl font-semibold tracking-tight", "Workflow — Status" }
                p { class: "text-sm text-muted-foreground", "Node execution status badges. Hit Run / Error / Reset." }
            }
            DemoWorkflowStatus {}

            div { class: "flex flex-col gap-1",
                h2 { class: "text-xl font-semibold tracking-tight", "Workflow — Minimap" }
                p { class: "text-sm text-muted-foreground",
                    "10-node pipeline spread across canvas. Minimap shows full graph at a glance."
                }
            }
            DemoWorkflowMinimap {}

            div { class: "flex flex-col gap-1",
                h2 { class: "text-xl font-semibold tracking-tight", "Workflow — Toolbar" }
                p { class: "text-sm text-muted-foreground",
                    "Toolbar overlay on canvas. Add nodes, undo/redo, reset zoom, delete selected."
                }
            }
            DemoWorkflowToolbar {}

            div { class: "flex flex-col gap-1",
                h2 { class: "text-xl font-semibold tracking-tight", "Workflow — Locked Mode" }
                p { class: "text-sm text-muted-foreground",
                    "Read-only canvas. Toggle lock to freeze all interactions — no drag, pan, zoom, or connect."
                }
            }
            DemoWorkflowLockedMode {}

            div { class: "flex flex-col gap-1",
                h2 { class: "text-xl font-semibold tracking-tight", "Workflow — Multi-select" }
                p { class: "text-sm text-muted-foreground",
                    "Shift+click to add nodes to selection. Drag moves all selected. Del removes all."
                }
            }
            DemoWorkflowMultiselect {}

            div { class: "flex flex-col gap-1",
                h2 { class: "text-xl font-semibold tracking-tight", "Workflow — Copy / Paste" }
                p { class: "text-sm text-muted-foreground",
                    "Select nodes, Ctrl+C to copy, Ctrl+V to paste (offset +20px each time)."
                }
            }
            DemoWorkflowCopyPaste {}

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
