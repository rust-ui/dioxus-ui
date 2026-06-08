use dioxus::prelude::*;
use icons::{Lock, LockOpen};
use registry::ui::toolbar::{Toolbar, ToolbarButton, ToolbarSeparator};

use super::node_canvas::{CanvasControls, DefaultNodeContent, NodeCanvas, NodeWrapper};
use crate::domain::test::hooks::use_node_canvas::{
    use_node_canvas, CanvasEdge, CanvasNode, NodeKind,
};

fn initial_nodes() -> Vec<CanvasNode> {
    vec![
        CanvasNode {
            id: "trigger".to_string(),
            initial_x: 40.0,
            initial_y: 150.0,
            width: 192.0,
            has_target: false,
            has_source: true,
            label: "User Input".to_string(),
            description: "Starts the workflow".to_string(),
            kind: NodeKind::Trigger,
        },
        CanvasNode {
            id: "agent".to_string(),
            initial_x: 280.0,
            initial_y: 150.0,
            width: 224.0,
            has_target: true,
            has_source: true,
            label: "AI Agent".to_string(),
            description: "claude-sonnet".to_string(),
            kind: NodeKind::Agent,
        },
        CanvasNode {
            id: "data".to_string(),
            initial_x: 280.0,
            initial_y: 280.0,
            width: 192.0,
            has_target: true,
            has_source: true,
            label: "Memory Store".to_string(),
            description: "Vector DB lookup".to_string(),
            kind: NodeKind::Data,
        },
        CanvasNode {
            id: "output".to_string(),
            initial_x: 560.0,
            initial_y: 150.0,
            width: 192.0,
            has_target: true,
            has_source: false,
            label: "Response".to_string(),
            description: "Ready to send".to_string(),
            kind: NodeKind::Output,
        },
    ]
}

fn initial_edges() -> Vec<CanvasEdge> {
    vec![
        CanvasEdge { from: "trigger".to_string(), to: "agent".to_string() },
        CanvasEdge { from: "data".to_string(),    to: "agent".to_string() },
        CanvasEdge { from: "agent".to_string(),   to: "output".to_string() },
    ]
}

#[component]
pub fn DemoNodeCanvasLockedMode() -> Element {
    let mut state = use_node_canvas(initial_nodes(), initial_edges());
    let locked = state.is_locked();

    rsx! {
        NodeCanvas {
            state,
            overlay: rsx! {
                // ── lock/unlock toolbar (top-center) ────────────────────────
                div {
                    class: "absolute top-3 left-1/2 -translate-x-1/2 z-20",
                    Toolbar { aria_label: "Lock controls",
                        ToolbarButton {
                            onclick: move |_| state.toggle_locked(),
                            if locked { Lock {} } else { LockOpen {} }
                            if locked { "Unlock Canvas" } else { "Lock Canvas" }
                        }
                        ToolbarSeparator {}
                        ToolbarButton {
                            onclick: move |_| state.zoom_reset(),
                            "Reset View"
                        }
                    }
                }

                // ── locked banner ────────────────────────────────────────────
                if locked {
                    div {
                        class: "absolute bottom-16 left-1/2 -translate-x-1/2 z-10 flex items-center gap-1.5 rounded-full bg-destructive/10 border border-destructive/20 px-3 py-1",
                        Lock { class: "size-3 text-destructive" }
                        span {
                            class: "text-xs text-destructive font-medium",
                            "Canvas locked — interactions disabled"
                        }
                    }
                }

                CanvasControls { state }
            },

            for (i, node) in state.nodes.read().iter().cloned().enumerate() {
                NodeWrapper { key: "{node.id}", state, idx: i,
                    DefaultNodeContent { node }
                }
            }
        }
    }
}
