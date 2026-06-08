use dioxus::prelude::*;
use icons::{Plus, Redo2, RotateCcw, Trash2, Undo2};
use registry::ui::toolbar::{
    Toolbar, ToolbarButton, ToolbarSeparator, ToolbarToggleGroup, ToolbarToggleItem,
};

use super::node_canvas::{CanvasControls, DefaultNodeContent, NodeCanvas, NodeWrapper};
use crate::domain::test::hooks::use_node_canvas::{
    use_node_canvas, CanvasEdge, CanvasNode, NodeKind,
};

fn initial_nodes() -> Vec<CanvasNode> {
    vec![
        CanvasNode {
            id: "trigger".to_string(),
            initial_x: 32.0,
            initial_y: 130.0,
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
            initial_y: 130.0,
            width: 224.0,
            has_target: true,
            has_source: true,
            label: "AI Agent".to_string(),
            description: "claude-sonnet".to_string(),
            kind: NodeKind::Agent,
        },
        CanvasNode {
            id: "output".to_string(),
            initial_x: 560.0,
            initial_y: 130.0,
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
        CanvasEdge { from: "agent".to_string(), to: "output".to_string() },
    ]
}

#[component]
pub fn DemoNodeCanvasToolbar() -> Element {
    let mut state = use_node_canvas(initial_nodes(), initial_edges());

    rsx! {
        NodeCanvas {
            state,
            overlay: rsx! {
                // ── canvas toolbar (top-center) ──────────────────────────────
                div {
                    class: "absolute top-3 left-1/2 -translate-x-1/2 z-20",
                    Toolbar { aria_label: "Canvas controls",
                        ToolbarButton {
                            onclick: move |_| state.add_node(200.0, 180.0),
                            Plus {}
                            "Add Node"
                        }
                        ToolbarSeparator {}
                        ToolbarToggleGroup {
                            ToolbarToggleItem {
                                title: "Undo",
                                onclick: move |_| state.undo(),
                                Undo2 {}
                            }
                            ToolbarToggleItem {
                                title: "Redo",
                                onclick: move |_| state.redo(),
                                Redo2 {}
                            }
                        }
                        ToolbarSeparator {}
                        ToolbarButton {
                            onclick: move |_| state.zoom_reset(),
                            RotateCcw {}
                            "Reset"
                        }
                        ToolbarButton {
                            onclick: move |_| state.delete_selected(),
                            Trash2 {}
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
