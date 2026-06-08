use dioxus::prelude::*;

use super::node_canvas::{CanvasControls, DefaultNodeContent, Minimap, NodeCanvas, NodeWrapper};
use crate::domain::test::hooks::use_node_canvas::{
    use_node_canvas, CanvasEdge, CanvasNode, NodeKind,
};

fn initial_nodes() -> Vec<CanvasNode> {
    vec![
        CanvasNode {
            id: "trigger".to_string(), initial_x: 32.0,  initial_y: 130.0, width: 192.0,
            has_target: false, has_source: true,
            label: "User Input".to_string(), description: "Starts the workflow".to_string(),
            kind: NodeKind::Trigger,
        },
        CanvasNode {
            id: "data".to_string(), initial_x: 272.0, initial_y: 40.0, width: 192.0,
            has_target: true, has_source: true,
            label: "Context".to_string(), description: "vectordb".to_string(),
            kind: NodeKind::Data,
        },
        CanvasNode {
            id: "agent".to_string(), initial_x: 272.0, initial_y: 160.0, width: 224.0,
            has_target: true, has_source: true,
            label: "AI Agent".to_string(), description: "claude-3.5-sonnet".to_string(),
            kind: NodeKind::Agent,
        },
        CanvasNode {
            id: "output".to_string(), initial_x: 552.0, initial_y: 130.0, width: 192.0,
            has_target: true, has_source: false,
            label: "Response".to_string(), description: "Ready to send".to_string(),
            kind: NodeKind::Output,
        },
    ]
}

fn initial_edges() -> Vec<CanvasEdge> {
    vec![
        CanvasEdge { from: "trigger".to_string(), to: "agent".to_string()  },
        CanvasEdge { from: "data".to_string(),    to: "agent".to_string()  },
        CanvasEdge { from: "agent".to_string(),   to: "output".to_string() },
    ]
}

#[component]
pub fn DemoNodeCanvas() -> Element {
    let state = use_node_canvas(initial_nodes(), initial_edges());

    rsx! {
        NodeCanvas {
            state,
            overlay: rsx! {
                CanvasControls { state }
                Minimap { state }
            },

            for (i, node) in state.nodes.read().iter().cloned().enumerate() {
                NodeWrapper { key: "{node.id}", state, idx: i, width: node.width,
                    DefaultNodeContent { node }
                }
            }
        }
    }
}
