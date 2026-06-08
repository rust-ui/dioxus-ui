use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct NodeData {
    pub id: &'static str,
    pub initial_x: f64,
    pub initial_y: f64,
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

#[derive(Clone, Copy)]
struct DragState {
    node_idx: usize,
    mouse_start_x: f64,
    mouse_start_y: f64,
    node_start_x: f64,
    node_start_y: f64,
}

const NODE_H: f64 = 72.0;

fn demo_nodes() -> Vec<NodeData> {
    vec![
        NodeData {
            id: "trigger",
            initial_x: 32.0,
            initial_y: 130.0,
            width: 192.0,
            label: "User Input",
            description: "Starts the workflow",
            kind: NodeKind::Trigger,
            has_target: false,
            has_source: true,
        },
        NodeData {
            id: "data",
            initial_x: 272.0,
            initial_y: 40.0,
            width: 192.0,
            label: "Context",
            description: "vectordb",
            kind: NodeKind::Data,
            has_target: true,
            has_source: true,
        },
        NodeData {
            id: "agent",
            initial_x: 272.0,
            initial_y: 160.0,
            width: 224.0,
            label: "AI Agent",
            description: "claude-3.5-sonnet",
            kind: NodeKind::Agent,
            has_target: true,
            has_source: true,
        },
        NodeData {
            id: "output",
            initial_x: 552.0,
            initial_y: 130.0,
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
    let edge_defs: &[(&str, &str)] = &[
        ("trigger", "agent"),
        ("data", "agent"),
        ("agent", "output"),
    ];

    let mut positions: Signal<Vec<(f64, f64)>> =
        use_signal(|| nodes.iter().map(|n| (n.initial_x, n.initial_y)).collect());

    let mut drag: Signal<Option<DragState>> = use_signal(|| None);

    // Snapshot for this render (avoids holding the read guard)
    let pos: Vec<(f64, f64)> = positions.read().clone();
    let drag_state = *drag.read();
    let is_dragging = drag_state.is_some();

    // Compute edge paths from current positions
    let edge_paths: Vec<String> = edge_defs
        .iter()
        .filter_map(|(from_id, to_id)| {
            let (fi, from) = nodes.iter().enumerate().find(|(_, n)| n.id == *from_id)?;
            let (ti, to) = nodes.iter().enumerate().find(|(_, n)| n.id == *to_id)?;
            let (fx, fy) = pos[fi];
            let (tx, ty) = pos[ti];
            let sx = fx + from.width;
            let sy = fy + NODE_H / 2.0;
            let tex = tx;
            let tey = ty + NODE_H / 2.0;
            Some(bezier_path(sx, sy, tex, tey))
        })
        .collect();

    rsx! {
        div {
            class: "relative rounded-lg border bg-background overflow-auto select-none",
            style: format!("height: 330px; cursor: {};", if is_dragging { "grabbing" } else { "default" }),

            onmousemove: move |ev| {
                if let Some(d) = *drag.read() {
                    let c = ev.data().client_coordinates();
                    let dx = c.x - d.mouse_start_x;
                    let dy = c.y - d.mouse_start_y;
                    let new_x = (d.node_start_x + dx).max(0.0);
                    let new_y = (d.node_start_y + dy).max(0.0);
                    positions.write()[d.node_idx] = (new_x, new_y);
                }
            },
            onmouseup: move |_| { drag.set(None); },
            onmouseleave: move |_| { drag.set(None); },

            div {
                class: "relative",
                style: "min-width: 790px; height: 310px;",

                // Background dot pattern
                div {
                    class: "absolute inset-0 pointer-events-none text-foreground",
                    style: "background-image: radial-gradient(circle, currentColor 1px, transparent 1px); background-size: 20px 20px; opacity: 0.12;",
                }

                // SVG edges
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
                for (idx, node) in nodes.iter().enumerate() {
                    div {
                        key: "{node.id}",
                        class: format!(
                            "absolute rounded-md border bg-card shadow-sm text-card-foreground transition-shadow{}",
                            if drag_state.map(|d| d.node_idx == idx).unwrap_or(false) {
                                " shadow-lg ring-1 ring-primary/20"
                            } else {
                                ""
                            }
                        ),
                        style: format!(
                            "left: {:.1}px; top: {:.1}px; width: {:.0}px; cursor: grab;",
                            pos[idx].0, pos[idx].1, node.width,
                        ),

                        onmousedown: move |ev| {
                            ev.stop_propagation();
                            let c = ev.data().client_coordinates();
                            let (nx, ny) = positions.read()[idx];
                            drag.set(Some(DragState {
                                node_idx: idx,
                                mouse_start_x: c.x,
                                mouse_start_y: c.y,
                                node_start_x: nx,
                                node_start_y: ny,
                            }));
                        },

                        if node.has_target {
                            div {
                                class: "absolute left-0 top-1/2 size-3 rounded-full border-2 border-primary bg-background pointer-events-none",
                                style: "transform: translate(-50%, -50%);",
                            }
                        }

                        if node.has_source {
                            div {
                                class: "absolute right-0 top-1/2 size-3 rounded-full border-2 border-primary bg-background pointer-events-none",
                                style: "transform: translate(50%, -50%);",
                            }
                        }

                        div {
                            class: "flex items-center gap-2 px-3 py-2.5 border-b bg-secondary/50 rounded-t-md",
                            span { class: format!("size-2 rounded-full shrink-0 {}", node.kind.dot_class()) }
                            span {
                                class: format!("text-xs font-medium {}", node.kind.text_class()),
                                "{node.label}"
                            }
                            span {
                                class: "ml-auto text-[10px] text-muted-foreground uppercase tracking-wide shrink-0",
                                "{node.kind.type_label()}"
                            }
                        }

                        div { class: "px-3 py-2",
                            p { class: "text-xs text-muted-foreground font-mono", "{node.description}" }
                        }
                    }
                }
            }
        }
    }
}
