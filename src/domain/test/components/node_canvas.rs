use dioxus::prelude::*;

use crate::domain::test::hooks::use_node_canvas::{CanvasEdge, CanvasNode, NodeCanvasState};

pub const NODE_H: f64 = 72.0;

#[component]
pub fn NodeCanvas(
    state: NodeCanvasState,
    nodes: Vec<CanvasNode>,
    edges: Vec<CanvasEdge>,
    children: Element,
) -> Element {
    let is_dragging = state.is_dragging();
    let edge_paths = state.edge_paths(&nodes, &edges, NODE_H);
    let mut state = state;

    rsx! {
        div {
            class: "relative rounded-lg border bg-background overflow-auto select-none",
            style: format!(
                "height: 330px; cursor: {};",
                if is_dragging { "grabbing" } else { "default" }
            ),

            onmousemove: move |ev| {
                let c = ev.data().client_coordinates();
                state.update_drag(c.x, c.y);
            },
            onmouseup: move |_| { state.stop_drag(); },
            onmouseleave: move |_| { state.stop_drag(); },

            div {
                class: "relative",
                style: "min-width: 790px; height: 310px;",

                div {
                    class: "absolute inset-0 pointer-events-none text-foreground",
                    style: "background-image: radial-gradient(circle, currentColor 1px, transparent 1px); background-size: 20px 20px; opacity: 0.12;",
                }

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

                {children}
            }
        }
    }
}

#[component]
pub fn NodeWrapper(
    state: NodeCanvasState,
    idx: usize,
    width: f64,
    children: Element,
) -> Element {
    let (x, y) = state.pos(idx);
    let is_active = state.active_idx() == Some(idx);
    let mut state = state;

    rsx! {
        div {
            class: format!(
                "absolute transition-shadow{}",
                if is_active { " shadow-lg" } else { "" }
            ),
            style: format!("left: {x:.1}px; top: {y:.1}px; width: {width:.0}px; cursor: grab;"),

            onmousedown: move |ev| {
                ev.stop_propagation();
                let c = ev.data().client_coordinates();
                state.start_drag(idx, c.x, c.y);
            },

            {children}
        }
    }
}
