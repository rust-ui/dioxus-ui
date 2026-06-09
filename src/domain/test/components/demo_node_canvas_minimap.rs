use dioxus::prelude::*;

use super::node_canvas::{CanvasControls, DefaultNodeContent, Minimap, NodeCanvas, NodeWrapper};
use crate::domain::test::hooks::use_node_canvas::{CanvasEdge, CanvasNode, NodeKind, use_node_canvas};

fn initial_nodes() -> Vec<CanvasNode> {
    vec![
        CanvasNode {
            id: "input".to_string(),
            initial_x: 40.0,
            initial_y: 300.0,
            width: 192.0,
            has_target: false,
            has_source: true,
            label: "User Input".to_string(),
            description: "Entry point".to_string(),
            kind: NodeKind::Trigger,
        },
        CanvasNode {
            id: "auth".to_string(),
            initial_x: 300.0,
            initial_y: 160.0,
            width: 192.0,
            has_target: true,
            has_source: true,
            label: "Auth Check".to_string(),
            description: "Validate token".to_string(),
            kind: NodeKind::Data,
        },
        CanvasNode {
            id: "cache".to_string(),
            initial_x: 300.0,
            initial_y: 300.0,
            width: 192.0,
            has_target: true,
            has_source: true,
            label: "Cache Lookup".to_string(),
            description: "Redis cache".to_string(),
            kind: NodeKind::Data,
        },
        CanvasNode {
            id: "retrieval".to_string(),
            initial_x: 300.0,
            initial_y: 440.0,
            width: 192.0,
            has_target: true,
            has_source: true,
            label: "Vector Search".to_string(),
            description: "pgvector".to_string(),
            kind: NodeKind::Data,
        },
        CanvasNode {
            id: "rerank".to_string(),
            initial_x: 560.0,
            initial_y: 370.0,
            width: 192.0,
            has_target: true,
            has_source: true,
            label: "Re-ranker".to_string(),
            description: "cross-encoder".to_string(),
            kind: NodeKind::Data,
        },
        CanvasNode {
            id: "agent".to_string(),
            initial_x: 820.0,
            initial_y: 260.0,
            width: 224.0,
            has_target: true,
            has_source: true,
            label: "AI Agent".to_string(),
            description: "claude-sonnet-4-6".to_string(),
            kind: NodeKind::Agent,
        },
        CanvasNode {
            id: "guard".to_string(),
            initial_x: 820.0,
            initial_y: 420.0,
            width: 192.0,
            has_target: true,
            has_source: true,
            label: "Safety Guard".to_string(),
            description: "content filter".to_string(),
            kind: NodeKind::Data,
        },
        CanvasNode {
            id: "formatter".to_string(),
            initial_x: 1100.0,
            initial_y: 300.0,
            width: 192.0,
            has_target: true,
            has_source: true,
            label: "Formatter".to_string(),
            description: "markdown → html".to_string(),
            kind: NodeKind::Data,
        },
        CanvasNode {
            id: "logger".to_string(),
            initial_x: 1100.0,
            initial_y: 460.0,
            width: 192.0,
            has_target: true,
            has_source: false,
            label: "Logger".to_string(),
            description: "audit trail".to_string(),
            kind: NodeKind::Output,
        },
        CanvasNode {
            id: "output".to_string(),
            initial_x: 1360.0,
            initial_y: 300.0,
            width: 192.0,
            has_target: true,
            has_source: false,
            label: "Response".to_string(),
            description: "stream to client".to_string(),
            kind: NodeKind::Output,
        },
    ]
}

fn initial_edges() -> Vec<CanvasEdge> {
    vec![
        CanvasEdge { from: "input".to_string(), to: "auth".to_string() },
        CanvasEdge { from: "input".to_string(), to: "cache".to_string() },
        CanvasEdge { from: "input".to_string(), to: "retrieval".to_string() },
        CanvasEdge { from: "retrieval".to_string(), to: "rerank".to_string() },
        CanvasEdge { from: "cache".to_string(), to: "agent".to_string() },
        CanvasEdge { from: "auth".to_string(), to: "agent".to_string() },
        CanvasEdge { from: "rerank".to_string(), to: "agent".to_string() },
        CanvasEdge { from: "agent".to_string(), to: "guard".to_string() },
        CanvasEdge { from: "agent".to_string(), to: "formatter".to_string() },
        CanvasEdge { from: "guard".to_string(), to: "logger".to_string() },
        CanvasEdge { from: "formatter".to_string(), to: "output".to_string() },
    ]
}

#[component]
pub fn DemoNodeCanvasMinimap() -> Element {
    let state = use_node_canvas(initial_nodes(), initial_edges());

    rsx! {
        NodeCanvas {
            state,
            overlay: rsx! {
                CanvasControls { state }
                Minimap { state }
            },
            for (i, node) in state.nodes.read().iter().cloned().enumerate() {
                NodeWrapper { key: "{node.id}", state, idx: i,
                    DefaultNodeContent { node }
                }
            }
        }
    }
}
