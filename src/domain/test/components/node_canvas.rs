use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct NodeData {
    pub id: &'static str,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub label: &'static str,
    pub description: &'static str,
    pub kind: NodeKind,
    pub has_target: bool,
    pub has_source: bool,
}

#[derive(Clone, Copy, PartialEq)]
pub enum NodeKind {
    Trigger,
    Data,
    Agent,
    Output,
}

impl NodeKind {
    fn dot_class(self) -> &'static str {
        match self {
            NodeKind::Trigger => "bg-yellow-500",
            NodeKind::Data => "bg-blue-500",
            NodeKind::Agent => "bg-purple-500",
            NodeKind::Output => "bg-green-500",
        }
    }

    fn text_class(self) -> &'static str {
        match self {
            NodeKind::Trigger => "text-yellow-600 dark:text-yellow-400",
            NodeKind::Data => "text-blue-600 dark:text-blue-400",
            NodeKind::Agent => "text-purple-600 dark:text-purple-400",
            NodeKind::Output => "text-green-600 dark:text-green-400",
        }
    }

    fn type_label(self) -> &'static str {
        match self {
            NodeKind::Trigger => "Trigger",
            NodeKind::Data => "Data",
            NodeKind::Agent => "Agent",
            NodeKind::Output => "Output",
        }
    }
}

// Approximate rendered card height (header + content)
const NODE_H: f64 = 72.0;

fn demo_nodes() -> Vec<NodeData> {
    vec![
        NodeData {
            id: "trigger",
            x: 32.0,
            y: 130.0,
            width: 192.0,
            label: "User Input",
            description: "Starts the workflow",
            kind: NodeKind::Trigger,
            has_target: false,
            has_source: true,
        },
        NodeData {
            id: "data",
            x: 272.0,
            y: 40.0,
            width: 192.0,
            label: "Context",
            description: "vectordb",
            kind: NodeKind::Data,
            has_target: true,
            has_source: true,
        },
        NodeData {
            id: "agent",
            x: 272.0,
            y: 160.0,
            width: 224.0,
            label: "AI Agent",
            description: "claude-3.5-sonnet",
            kind: NodeKind::Agent,
            has_target: true,
            has_source: true,
        },
        NodeData {
            id: "output",
            x: 552.0,
            y: 130.0,
            width: 192.0,
            label: "Response",
            description: "Ready to send",
            kind: NodeKind::Output,
            has_target: true,
            has_source: false,
        },
    ]
}

fn bezier_path(sx: f64, sy: f64, tx: f64, ty: f64) -> String {
    let dx = (tx - sx).abs();
    let offset = (dx / 2.0).clamp(40.0, 80.0);
    format!(
        "M {sx:.1} {sy:.1} C {:.1} {sy:.1}, {:.1} {ty:.1}, {tx:.1} {ty:.1}",
        sx + offset,
        tx - offset,
    )
}

#[component]
pub fn NodeCanvas() -> Element {
    let nodes = demo_nodes();
    let edges: Vec<(&str, &str)> = vec![
        ("trigger", "agent"),
        ("data", "agent"),
        ("agent", "output"),
    ];

    let edge_paths: Vec<String> = edges
        .iter()
        .filter_map(|(from_id, to_id)| {
            let from = nodes.iter().find(|n| n.id == *from_id)?;
            let to = nodes.iter().find(|n| n.id == *to_id)?;
            let sx = from.x + from.width;
            let sy = from.y + NODE_H / 2.0;
            let tx = to.x;
            let ty = to.y + NODE_H / 2.0;
            Some(bezier_path(sx, sy, tx, ty))
        })
        .collect();

    rsx! {
        div {
            class: "relative rounded-lg border bg-background overflow-auto select-none",
            style: "height: 330px;",

            div {
                class: "relative",
                style: "min-width: 790px; height: 310px;",

                // Background dot pattern
                div {
                    class: "absolute inset-0 pointer-events-none text-foreground",
                    style: "background-image: radial-gradient(circle, currentColor 1px, transparent 1px); background-size: 20px 20px; opacity: 0.12;",
                }

                // SVG layer — edges
                svg {
                    class: "absolute inset-0 w-full h-full pointer-events-none overflow-visible",
                    for d in &edge_paths {
                        path {
                            d: d.as_str(),
                            fill: "none",
                            stroke: "currentColor",
                            class: "text-border",
                            "stroke-width": "1.5",
                            "stroke-linecap": "round",
                        }
                    }
                }

                // Nodes
                for node in &nodes {
                    div {
                        key: "{node.id}",
                        class: "absolute rounded-md border bg-card shadow-sm text-card-foreground",
                        style: format!(
                            "left: {:.0}px; top: {:.0}px; width: {:.0}px;",
                            node.x, node.y, node.width,
                        ),

                        if node.has_target {
                            div {
                                class: "absolute left-0 top-1/2 size-3 rounded-full border-2 border-primary bg-background",
                                style: "transform: translate(-50%, -50%);",
                            }
                        }

                        if node.has_source {
                            div {
                                class: "absolute right-0 top-1/2 size-3 rounded-full border-2 border-primary bg-background",
                                style: "transform: translate(50%, -50%);",
                            }
                        }

                        // Header
                        div {
                            class: "flex items-center gap-2 px-3 py-2.5 border-b bg-secondary/50 rounded-t-md",
                            span {
                                class: format!("size-2 rounded-full shrink-0 {}", node.kind.dot_class()),
                            }
                            span {
                                class: format!("text-xs font-medium {}", node.kind.text_class()),
                                "{node.label}"
                            }
                            span {
                                class: "ml-auto text-[10px] text-muted-foreground uppercase tracking-wide shrink-0",
                                "{node.kind.type_label()}"
                            }
                        }

                        // Content
                        div { class: "px-3 py-2",
                            p { class: "text-xs text-muted-foreground font-mono", "{node.description}" }
                        }
                    }
                }
            }
        }
    }
}
