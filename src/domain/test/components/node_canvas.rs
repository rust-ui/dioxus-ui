use dioxus::prelude::*;
use dioxus_html::geometry::WheelDelta;
use dioxus_html::input_data::keyboard_types::{Key, Modifiers};

use crate::domain::test::hooks::use_node_canvas::{CanvasEdge, CanvasNode, NodeCanvasState};

const VIEWPORT_W: f64 = 800.0;
const VIEWPORT_H: f64 = 450.0;
const MINI_W: f64 = 140.0;
const MINI_H: f64 = 90.0;
const WORLD_REF_W: f64 = 1600.0;
const WORLD_REF_H: f64 = 900.0;

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
    #[props(optional)] overlay: Option<Element>,
) -> Element {
    let is_busy = state.is_dragging() || state.is_panning();
    let edge_paths = state.edge_paths(&nodes, &edges, NODE_H);
    let transform = state.world_transform();
    let mut state = state;

    rsx! {
        div {
            class: "relative rounded-lg border bg-background overflow-hidden select-none outline-none",
            style: format!("height: 450px; cursor: {};", if is_busy { "grabbing" } else { "grab" }),
            tabindex: "0",

            onkeydown: move |ev| {
                let data = ev.data();
                let mods = data.modifiers();
                let ctrl = mods.contains(Modifiers::CONTROL) || mods.contains(Modifiers::META);
                let shift = mods.contains(Modifiers::SHIFT);
                match data.key() {
                    Key::Character(ref k) if ctrl && k == "z" && !shift => {
                        ev.prevent_default();
                        state.undo();
                    }
                    Key::Character(ref k) if ctrl && (k == "y" || (k == "z" && shift)) => {
                        ev.prevent_default();
                        state.redo();
                    }
                    _ => {}
                }
            },

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
                    style { "@keyframes edge-flow {{ to {{ stroke-dashoffset: -18; }} }}" }
                    for d in &edge_paths {
                        path {
                            d: d.as_str(),
                            fill: "none",
                            stroke: "currentColor",
                            class: "text-border",
                            "stroke-width": "1.5",
                            "stroke-linecap": "round",
                            "stroke-dasharray": "6 3",
                            style: "animation: edge-flow 1.2s linear infinite;",
                        }
                    }
                }

                {children}
            }

            // ── overlays (viewport space, not transformed) ───────────────────
            {overlay}
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

// ── Minimap ───────────────────────────────────────────────────────────────────

#[component]
pub fn Minimap(
    state: NodeCanvasState,
    nodes: Vec<CanvasNode>,
    edges: Vec<CanvasEdge>,
) -> Element {
    let scale_x = MINI_W / WORLD_REF_W;
    let scale_y = MINI_H / WORLD_REF_H;

    let pos_snap = state.positions.read().clone();

    // precompute node rects: (x, y, w, h) in minimap coords
    let node_rects: Vec<(f64, f64, f64, f64)> = nodes
        .iter()
        .enumerate()
        .map(|(i, n)| {
            let (x, y) = pos_snap[i];
            (x * scale_x, y * scale_y, n.width * scale_x, NODE_H * scale_y)
        })
        .collect();

    // precompute scaled bezier edge paths
    let edge_paths: Vec<String> = edges
        .iter()
        .filter_map(|edge| {
            let (fi, from) = nodes.iter().enumerate().find(|(_, n)| n.id == edge.from)?;
            let (ti, _) = nodes.iter().enumerate().find(|(_, n)| n.id == edge.to)?;
            let (fx, fy) = pos_snap[fi];
            let (tx, ty) = pos_snap[ti];
            let sx = (fx + from.width) * scale_x;
            let sy = (fy + NODE_H / 2.0) * scale_y;
            let tx2 = tx * scale_x;
            let ty2 = (ty + NODE_H / 2.0) * scale_y;
            let dx = (tx2 - sx).abs();
            let off = (dx / 2.0).clamp(4.0, 12.0);
            Some(format!(
                "M {sx:.1} {sy:.1} C {:.1} {sy:.1}, {:.1} {ty2:.1}, {tx2:.1} {ty2:.1}",
                sx + off,
                tx2 - off,
            ))
        })
        .collect();

    // viewport indicator rect in minimap coords
    let (pan_x, pan_y) = *state.pan.read();
    let zoom = state.zoom_value();
    let vp_x = (-pan_x / zoom) * scale_x;
    let vp_y = (-pan_y / zoom) * scale_y;
    let vp_w = (VIEWPORT_W / zoom) * scale_x;
    let vp_h = (VIEWPORT_H / zoom) * scale_y;

    rsx! {
        div {
            class: "rounded-md border bg-background/90 backdrop-blur-sm shadow-sm overflow-hidden pointer-events-none",
            style: format!(
                "position: absolute; bottom: 12px; right: 12px; width: {MINI_W}px; height: {MINI_H}px;"
            ),

            svg {
                width: "{MINI_W}",
                height: "{MINI_H}",

                // edges
                for d in &edge_paths {
                    path {
                        d: d.as_str(),
                        fill: "none",
                        stroke: "hsl(var(--border))",
                        "stroke-width": "0.8",
                    }
                }

                // node rects
                for (x, y, w, h) in &node_rects {
                    rect {
                        x: "{x:.1}",
                        y: "{y:.1}",
                        width: "{w:.1}",
                        height: "{h:.1}",
                        rx: "1.5",
                        fill: "hsl(var(--muted-foreground) / 0.3)",
                        stroke: "hsl(var(--muted-foreground) / 0.6)",
                        "stroke-width": "0.5",
                    }
                }

                // viewport indicator
                rect {
                    x: "{vp_x:.1}",
                    y: "{vp_y:.1}",
                    width: "{vp_w:.1}",
                    height: "{vp_h:.1}",
                    rx: "2",
                    fill: "hsl(var(--primary) / 0.08)",
                    stroke: "hsl(var(--primary) / 0.5)",
                    "stroke-width": "1",
                    "stroke-dasharray": "2 1",
                }
            }
        }
    }
}

// ── CanvasControls ────────────────────────────────────────────────────────────

#[component]
pub fn CanvasControls(state: NodeCanvasState, nodes: Vec<CanvasNode>) -> Element {
    let pct = (state.zoom_value() * 100.0).round() as i32;
    let mut state = state;

    rsx! {
        div {
            class: "flex items-center gap-0.5 rounded-md border bg-background/90 backdrop-blur-sm shadow-sm px-1.5 py-1",
            style: "position: absolute; top: 12px; right: 12px;",

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
                onclick: move |_| { state.fit_to_view(&nodes, VIEWPORT_W, VIEWPORT_H, NODE_H); },
                "fit"
            }
            button {
                class: "text-[11px] px-1.5 py-0.5 rounded hover:bg-accent text-muted-foreground transition-colors",
                onclick: move |_| { state.zoom_reset(); },
                "reset"
            }
        }
    }
}
