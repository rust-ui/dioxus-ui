use dioxus::prelude::*;
use dioxus_html::geometry::WheelDelta;

use crate::domain::test::hooks::use_node_canvas::{CanvasEdge, CanvasNode, NodeCanvasState};

pub const NODE_H: f64 = 72.0;

// ── NodeCanvas ────────────────────────────────────────────────────────────────
//
// Viewport div (overflow:hidden, fixed height)
// └── world div (3000×2000, CSS transform: translate+scale)
//     ├── dot background
//     ├── SVG bezier edges
//     └── children (NodeWrapper instances)
// └── ZoomControls overlay (not transformed — stays in viewport space)

#[component]
pub fn NodeCanvas(
    state: NodeCanvasState,
    nodes: Vec<CanvasNode>,
    edges: Vec<CanvasEdge>,
    children: Element,
) -> Element {
    let is_busy = state.is_dragging() || state.is_panning();
    let edge_paths = state.edge_paths(&nodes, &edges, NODE_H);
    let transform = state.world_transform();
    let mut state = state;

    rsx! {
        div {
            class: "relative rounded-lg border bg-background overflow-hidden select-none",
            style: format!("height: 450px; cursor: {};", if is_busy { "grabbing" } else { "grab" }),

            // canvas pan on background click (nodes stop propagation so only fires on empty space)
            onmousedown: move |ev| {
                let c = ev.data().client_coordinates();
                state.start_pan(c.x, c.y);
            },
            onmousemove: move |ev| {
                let c = ev.data().client_coordinates();
                state.update_drag(c.x, c.y);
                state.update_pan(c.x, c.y);
            },
            onmouseup: move |_| { state.stop_drag(); state.stop_pan(); },
            onmouseleave: move |_| { state.stop_drag(); state.stop_pan(); },
            onwheel: move |ev| {
                ev.prevent_default();
                let pos = ev.data().element_coordinates();
                let delta_y = match ev.data().delta() {
                    WheelDelta::Pixels(p) => p.y,
                    WheelDelta::Lines(p) => p.y * 20.0,
                    WheelDelta::Pages(p) => p.y * 400.0,
                };
                state.zoom_at(pos.x, pos.y, delta_y);
            },

            // ── world (transformed) ──────────────────────────────────────────
            div {
                style: format!(
                    "position: absolute; top: 0; left: 0; width: 3000px; height: 2000px; transform: {transform}; transform-origin: 0 0;"
                ),

                // dot grid background
                div {
                    class: "absolute inset-0 pointer-events-none text-foreground",
                    style: "background-image: radial-gradient(circle, currentColor 1px, transparent 1px); background-size: 20px 20px; opacity: 0.12;",
                }

                // bezier edges
                svg {
                    class: "absolute inset-0 pointer-events-none overflow-visible",
                    width: "3000",
                    height: "2000",
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

            // ── zoom controls (viewport space, not transformed) ───────────────
            ZoomControls { state }
        }
    }
}

// ── NodeWrapper ───────────────────────────────────────────────────────────────

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

// ── ZoomControls ──────────────────────────────────────────────────────────────

#[component]
pub fn ZoomControls(state: NodeCanvasState) -> Element {
    let pct = (state.zoom_value() * 100.0).round() as i32;
    let mut state = state;

    rsx! {
        div {
            class: "absolute bottom-3 right-3 flex items-center gap-0.5 rounded-md border bg-background/90 backdrop-blur-sm shadow-sm px-1.5 py-1",

            span {
                class: "text-[11px] text-muted-foreground tabular-nums w-9 text-center",
                "{pct}%"
            }

            button {
                class: "size-6 flex items-center justify-center rounded text-sm hover:bg-accent transition-colors",
                onclick: move |_| { state.zoom_step(1.0 / 1.2); },
                "−"
            }
            button {
                class: "size-6 flex items-center justify-center rounded text-sm hover:bg-accent transition-colors",
                onclick: move |_| { state.zoom_step(1.2); },
                "+"
            }
            div { class: "w-px h-4 bg-border mx-0.5" }
            button {
                class: "text-[11px] px-1.5 py-0.5 rounded hover:bg-accent text-muted-foreground transition-colors",
                onclick: move |_| { state.zoom_reset(); },
                "reset"
            }
        }
    }
}
