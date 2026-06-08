use dioxus::prelude::*;

use super::node::{Node, NodeContent, NodeDescription, NodeHeader, NodeTitle};
use super::node_canvas::{CanvasControls, Minimap, NodeCanvas, NodeWrapper};
use crate::domain::test::hooks::use_node_canvas::{
    use_node_canvas, CanvasEdge, CanvasNode,
};

fn nodes() -> Vec<CanvasNode> {
    vec![
        CanvasNode { id: "trigger", initial_x: 32.0,  initial_y: 130.0, width: 192.0, has_target: false, has_source: true  },
        CanvasNode { id: "data",    initial_x: 272.0, initial_y: 40.0,  width: 192.0, has_target: true,  has_source: true  },
        CanvasNode { id: "agent",   initial_x: 272.0, initial_y: 160.0, width: 224.0, has_target: true,  has_source: true  },
        CanvasNode { id: "output",  initial_x: 552.0, initial_y: 130.0, width: 192.0, has_target: true,  has_source: false },
    ]
}

fn edges() -> Vec<CanvasEdge> {
    vec![
        CanvasEdge { from: "trigger", to: "agent"  },
        CanvasEdge { from: "data",    to: "agent"  },
        CanvasEdge { from: "agent",   to: "output" },
    ]
}

#[component]
pub fn DemoNodeCanvas() -> Element {
    let ns = nodes();
    let state = use_node_canvas(&ns);

    rsx! {
        NodeCanvas {
            state,
            nodes: ns,
            edges: edges(),
            overlay: rsx! {
                CanvasControls { state, nodes: nodes() }
                Minimap { state, nodes: nodes(), edges: edges() }
            },

            // 0 — Trigger node (no input handle, one output)
            NodeWrapper { state, idx: 0, width: 192.0,
                Node { source: true,
                    NodeHeader {
                        span { class: "size-2 rounded-full bg-yellow-500 shrink-0" }
                        NodeTitle { class: "text-yellow-600 dark:text-yellow-400", "User Input" }
                        span { class: "ml-auto text-[10px] text-muted-foreground uppercase tracking-wide shrink-0", "Trigger" }
                    }
                    NodeContent {
                        NodeDescription { "Starts the workflow" }
                    }
                }
            }

            // 1 — Data / RAG node
            NodeWrapper { state, idx: 1, width: 192.0,
                Node { target: true, source: true,
                    NodeHeader {
                        span { class: "size-2 rounded-full bg-blue-500 shrink-0" }
                        NodeTitle { class: "text-blue-600 dark:text-blue-400", "Context" }
                        span { class: "ml-auto text-[10px] text-muted-foreground uppercase tracking-wide shrink-0", "Data" }
                    }
                    NodeContent {
                        NodeDescription { class: "font-mono", "vectordb" }
                    }
                }
            }

            // 2 — Agent node (two inputs, one output)
            NodeWrapper { state, idx: 2, width: 224.0,
                Node { target: true, source: true,
                    NodeHeader {
                        span { class: "size-2 rounded-full bg-purple-500 shrink-0" }
                        NodeTitle { class: "text-purple-600 dark:text-purple-400", "AI Agent" }
                        span { class: "ml-auto text-[10px] text-muted-foreground uppercase tracking-wide shrink-0", "Agent" }
                    }
                    NodeContent {
                        NodeDescription { class: "font-mono", "claude-3.5-sonnet" }
                    }
                }
            }

            // 3 — Output node (input only)
            NodeWrapper { state, idx: 3, width: 192.0,
                Node { target: true,
                    NodeHeader {
                        span { class: "size-2 rounded-full bg-green-500 shrink-0" }
                        NodeTitle { class: "text-green-600 dark:text-green-400", "Response" }
                        span { class: "ml-auto text-[10px] text-muted-foreground uppercase tracking-wide shrink-0", "Output" }
                    }
                    NodeContent {
                        NodeDescription { "Ready to send" }
                    }
                }
            }
        }
    }
}
