use std::collections::HashMap;

use dioxus::prelude::*;

use super::node::{Node, NodeContent, NodeDescription, NodeFooter, NodeHeader, NodeTitle};
use super::node_canvas::{CanvasControls, Minimap, NodeCanvas, NodeWrapper};
use crate::domain::test::hooks::use_node_canvas::{
    use_node_canvas, CanvasEdge, CanvasNode, NodeKind, NodeCanvasState,
};

// ── NodeStatus ────────────────────────────────────────────────────────────────

#[derive(Clone, PartialEq, Default)]
pub enum NodeStatus {
    #[default]
    Idle,
    Queued,
    Running,
    Success,
    Failed,
}

impl NodeStatus {
    fn label(&self) -> &'static str {
        match self {
            NodeStatus::Idle    => "Idle",
            NodeStatus::Queued  => "Queued",
            NodeStatus::Running => "In progress",
            NodeStatus::Success => "Success",
            NodeStatus::Failed  => "Failed",
        }
    }

    // Left border color — the main GH Actions visual cue
    fn border_class(&self) -> &'static str {
        match self {
            NodeStatus::Idle    => "border-l-zinc-200 dark:border-l-zinc-700",
            NodeStatus::Queued  => "border-l-amber-400",
            NodeStatus::Running => "border-l-orange-400",
            NodeStatus::Success => "border-l-emerald-500",
            NodeStatus::Failed  => "border-l-red-500",
        }
    }

    // Subtle background tint
    fn bg_class(&self) -> &'static str {
        match self {
            NodeStatus::Idle    => "",
            NodeStatus::Queued  => "bg-amber-50/40 dark:bg-amber-950/10",
            NodeStatus::Running => "bg-orange-50/40 dark:bg-orange-950/10",
            NodeStatus::Success => "bg-emerald-50/40 dark:bg-emerald-950/10",
            NodeStatus::Failed  => "bg-red-50/40 dark:bg-red-950/10",
        }
    }

    // Icon color
    fn icon_color(&self) -> &'static str {
        match self {
            NodeStatus::Idle    => "text-zinc-400",
            NodeStatus::Queued  => "text-amber-500",
            NodeStatus::Running => "text-orange-500",
            NodeStatus::Success => "text-emerald-500",
            NodeStatus::Failed  => "text-red-500",
        }
    }

    // Label color
    fn label_color(&self) -> &'static str {
        match self {
            NodeStatus::Idle    => "text-zinc-400",
            NodeStatus::Queued  => "text-amber-600 dark:text-amber-400",
            NodeStatus::Running => "text-orange-600 dark:text-orange-400",
            NodeStatus::Success => "text-emerald-600 dark:text-emerald-400",
            NodeStatus::Failed  => "text-red-600 dark:text-red-400",
        }
    }
}

// ── Status icons (inline SVG) ─────────────────────────────────────────────────

#[component]
fn StatusIcon(status: NodeStatus) -> Element {
    let color = status.icon_color();
    match status {
        NodeStatus::Idle => rsx! {
            svg {
                class: "size-3.5 {color}",
                view_box: "0 0 16 16", fill: "none",
                circle { cx: "8", cy: "8", r: "6", stroke: "currentColor", "stroke-width": "1.5" }
            }
        },
        NodeStatus::Queued => rsx! {
            svg {
                class: "size-3.5 {color}",
                view_box: "0 0 16 16", fill: "none",
                circle { cx: "8", cy: "8", r: "6.5", stroke: "currentColor", "stroke-width": "1.5" }
                // clock hands
                path { d: "M8 5v3.5l2 1.5", stroke: "currentColor", "stroke-width": "1.5", "stroke-linecap": "round" }
            }
        },
        NodeStatus::Running => rsx! {
            svg {
                class: "size-3.5 {color}",
                style: "animation: spin 1s linear infinite;",
                view_box: "0 0 16 16", fill: "none",
                circle {
                    cx: "8", cy: "8", r: "6",
                    stroke: "currentColor", "stroke-width": "1.5",
                    "stroke-dasharray": "20 18",
                    "stroke-linecap": "round",
                }
            }
        },
        NodeStatus::Success => rsx! {
            svg {
                class: "size-3.5 {color}",
                view_box: "0 0 16 16", fill: "none",
                circle { cx: "8", cy: "8", r: "6.5", fill: "currentColor", opacity: "0.15" }
                circle { cx: "8", cy: "8", r: "6.5", stroke: "currentColor", "stroke-width": "1.5" }
                path { d: "M5.5 8.5l2 2 3-3.5", stroke: "currentColor", "stroke-width": "1.5", "stroke-linecap": "round", "stroke-linejoin": "round" }
            }
        },
        NodeStatus::Failed => rsx! {
            svg {
                class: "size-3.5 {color}",
                view_box: "0 0 16 16", fill: "none",
                circle { cx: "8", cy: "8", r: "6.5", fill: "currentColor", opacity: "0.15" }
                circle { cx: "8", cy: "8", r: "6.5", stroke: "currentColor", "stroke-width": "1.5" }
                path { d: "M5.5 5.5l5 5M10.5 5.5l-5 5", stroke: "currentColor", "stroke-width": "1.5", "stroke-linecap": "round" }
            }
        },
    }
}

// ── StatusNodeContent ─────────────────────────────────────────────────────────

#[component]
fn StatusNodeContent(node: CanvasNode, status: NodeStatus) -> Element {
    let border = status.border_class();
    let bg     = status.bg_class();
    let lcolor = status.label_color();
    let slabel = status.label();

    rsx! {
        // inject spin keyframe once
        style { "@keyframes spin {{ to {{ transform: rotate(360deg); }} }}" }

        Node {
            target: false,
            source: false,
            class: format!("border-l-[3px] {border} {bg}"),
            NodeHeader {
                span { class: format!("size-2 rounded-full shrink-0 {}", node.kind.dot_color()) }
                NodeTitle { class: node.kind.text_color(), "{node.label}" }
                span {
                    class: "ml-auto text-[10px] text-muted-foreground uppercase tracking-wide shrink-0",
                    "{node.kind.label()}"
                }
            }
            NodeContent {
                NodeDescription { class: "font-mono", "{node.description}" }
            }
            NodeFooter {
                class: "py-1.5",
                StatusIcon { status: status.clone() }
                span { class: format!("text-[11px] font-medium {lcolor}"), "{slabel}" }
            }
        }
    }
}

// ── StatusOverlay ─────────────────────────────────────────────────────────────

#[component]
fn StatusOverlay(
    statuses: Signal<HashMap<String, NodeStatus>>,
    state: NodeCanvasState,
) -> Element {
    let run_sequence = move |_| {
        let mut s = statuses.clone();
        s.write().clear();
        s.write().insert("trigger".to_string(), NodeStatus::Success);
        s.write().insert("data".to_string(),    NodeStatus::Success);
        s.write().insert("agent".to_string(),   NodeStatus::Running);
        s.write().insert("output".to_string(),  NodeStatus::Queued);
    };

    let set_error = move |_| {
        let mut s = statuses.clone();
        s.write().insert("trigger".to_string(), NodeStatus::Success);
        s.write().insert("data".to_string(),    NodeStatus::Success);
        s.write().insert("agent".to_string(),   NodeStatus::Failed);
        s.write().insert("output".to_string(),  NodeStatus::Idle);
    };

    let set_done = move |_| {
        let mut s = statuses.clone();
        s.write().insert("trigger".to_string(), NodeStatus::Success);
        s.write().insert("data".to_string(),    NodeStatus::Success);
        s.write().insert("agent".to_string(),   NodeStatus::Success);
        s.write().insert("output".to_string(),  NodeStatus::Success);
    };

    let reset = move |_| { statuses.clone().write().clear(); };

    rsx! {
        div {
            class: "absolute bottom-3 left-3 flex items-center gap-1 rounded-lg border bg-background/95 backdrop-blur px-1.5 py-1.5 shadow-sm",

            // Run button — orange like GH Actions
            button {
                class: "flex items-center gap-1.5 rounded px-2.5 py-1 text-[11px] font-semibold bg-orange-500 hover:bg-orange-400 text-white transition-colors",
                onclick: run_sequence,
                svg {
                    class: "size-3",
                    view_box: "0 0 16 16", fill: "currentColor",
                    path { d: "M4 2l10 6-10 6V2z" }
                }
                "Run"
            }
            button {
                class: "flex items-center gap-1.5 rounded px-2 py-1 text-[11px] font-medium bg-emerald-600 hover:bg-emerald-500 text-white transition-colors",
                onclick: set_done,
                "Done"
            }
            button {
                class: "flex items-center gap-1.5 rounded px-2 py-1 text-[11px] font-medium bg-red-600 hover:bg-red-500 text-white transition-colors",
                onclick: set_error,
                "Fail"
            }
            div { class: "w-px h-4 bg-border mx-0.5" }
            button {
                class: "rounded px-2 py-1 text-[11px] text-muted-foreground hover:bg-accent transition-colors",
                onclick: reset,
                "Reset"
            }
        }

        CanvasControls { state }
        Minimap { state }
    }
}

// ── Demo ──────────────────────────────────────────────────────────────────────

fn initial_nodes() -> Vec<CanvasNode> {
    vec![
        CanvasNode {
            id: "trigger".to_string(), initial_x: 32.0, initial_y: 150.0, width: 200.0,
            has_target: false, has_source: true,
            label: "Webhook".to_string(), description: "POST /api/run".to_string(),
            kind: NodeKind::Trigger,
        },
        CanvasNode {
            id: "data".to_string(), initial_x: 288.0, initial_y: 50.0, width: 200.0,
            has_target: true, has_source: true,
            label: "Vector DB".to_string(), description: "pgvector lookup".to_string(),
            kind: NodeKind::Data,
        },
        CanvasNode {
            id: "agent".to_string(), initial_x: 288.0, initial_y: 188.0, width: 224.0,
            has_target: true, has_source: true,
            label: "AI Agent".to_string(), description: "claude-sonnet-4-6".to_string(),
            kind: NodeKind::Agent,
        },
        CanvasNode {
            id: "output".to_string(), initial_x: 572.0, initial_y: 150.0, width: 200.0,
            has_target: true, has_source: false,
            label: "Response".to_string(), description: "stream to client".to_string(),
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
pub fn DemoNodeCanvasStatus() -> Element {
    let state    = use_node_canvas(initial_nodes(), initial_edges());
    let statuses = use_signal(HashMap::<String, NodeStatus>::new);

    rsx! {
        NodeCanvas {
            state,
            overlay: rsx! { StatusOverlay { statuses, state } },
            for (i, node) in state.nodes.read().iter().cloned().enumerate() {
                NodeWrapper { key: "{node.id}", state, idx: i,
                    StatusNodeContent {
                        node: node.clone(),
                        status: statuses.read().get(&node.id).cloned().unwrap_or_default(),
                    }
                }
            }
        }
    }
}
