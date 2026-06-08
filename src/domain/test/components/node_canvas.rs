use dioxus::prelude::*;
use dioxus_html::geometry::WheelDelta;
use dioxus_html::input_data::keyboard_types::{Key, Modifiers};

use crate::domain::test::hooks::use_node_canvas::{CanvasNode, NodeCanvasState};

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
// └── overlay (viewport space — CanvasControls, Minimap, etc.)

#[component]
pub fn NodeCanvas(
    state: NodeCanvasState,
    children: Element,
    #[props(optional)] overlay: Option<Element>,
) -> Element {
    let is_busy   = state.is_dragging() || state.is_panning();
    let locked    = state.is_locked();
    let edge_paths = state.edge_paths(NODE_H);
    let transform = state.world_transform();
    let mut state = state;
    // Top-left of the canvas element in the viewport — used to convert touch
    // client coordinates to canvas-space coordinates for pinch-zoom pivot.
    let mut canvas_origin = use_signal(|| (0.0_f64, 0.0_f64));

    rsx! {
        div {
            class: "relative rounded-lg border bg-background overflow-hidden select-none outline-none",
            // touch-action: none — tells the browser we handle all touch gestures
            // ourselves, preventing native scroll/zoom from fighting our handlers.
            style: format!(
                "height: 450px; cursor: {}; touch-action: none;",
                if locked { "default" } else if is_busy { "grabbing" } else { "grab" }
            ),
            tabindex: "0",

            onmounted: move |data| {
                spawn(async move {
                    if let Ok(rect) = data.get_client_rect().await {
                        canvas_origin.set((rect.min_x(), rect.min_y()));
                    }
                });
            },

            onkeydown: move |ev| {
                if locked { return; }
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
                    Key::Delete | Key::Backspace => {
                        ev.prevent_default();
                        state.delete_selected();
                    }
                    Key::Escape => { state.deselect(); }
                    _ => {}
                }
            },

            onmousedown: move |ev| {
                if locked { return; }
                let c = ev.data().client_coordinates();
                state.deselect();
                if state.is_connecting() {
                    state.cancel_connect();
                } else {
                    state.start_pan(c.x, c.y);
                }
            },
            onmousemove: move |ev| {
                if locked { return; }
                let c = ev.data().client_coordinates();
                state.update_drag(c.x, c.y);
                state.update_pan(c.x, c.y);
                let ec = ev.data().element_coordinates();
                state.update_connect_mouse(ec.x, ec.y);
            },
            onmouseup: move |_| { state.stop_drag(); state.stop_pan(); state.cancel_connect(); },
            onmouseleave: move |_| { state.stop_drag(); state.stop_pan(); state.cancel_connect(); },
            onwheel: move |ev| {
                if locked { return; }
                ev.prevent_default();
                let pos = ev.data().element_coordinates();
                let delta_y = match ev.data().delta() {
                    WheelDelta::Pixels(p) => p.y,
                    WheelDelta::Lines(p)  => p.y * 20.0,
                    WheelDelta::Pages(p)  => p.y * 400.0,
                };
                state.zoom_at(pos.x, pos.y, delta_y);
            },

            // ── touch (single-finger pan + two-finger pinch-zoom) ────────────
            ontouchstart: move |ev| {
                if locked { return; }
                ev.prevent_default();
                let touches = ev.data().touches();
                match touches.len() {
                    1 => {
                        state.stop_pinch();
                        state.deselect();
                        state.start_pan(touches[0].client_x(), touches[0].client_y());
                    }
                    2 => {
                        state.stop_pan();
                        let dist = touch_dist(
                            touches[0].client_x(), touches[0].client_y(),
                            touches[1].client_x(), touches[1].client_y(),
                        );
                        let (ox, oy) = *canvas_origin.read();
                        let cx = (touches[0].client_x() + touches[1].client_x()) / 2.0 - ox;
                        let cy = (touches[0].client_y() + touches[1].client_y()) / 2.0 - oy;
                        state.start_pinch(dist, cx, cy);
                    }
                    _ => {}
                }
            },
            ontouchmove: move |ev| {
                if locked { return; }
                ev.prevent_default();
                let touches = ev.data().touches();
                match touches.len() {
                    1 if !state.is_pinching() => {
                        state.update_pan(touches[0].client_x(), touches[0].client_y());
                    }
                    2 => {
                        let dist = touch_dist(
                            touches[0].client_x(), touches[0].client_y(),
                            touches[1].client_x(), touches[1].client_y(),
                        );
                        let (ox, oy) = *canvas_origin.read();
                        let cx = (touches[0].client_x() + touches[1].client_x()) / 2.0 - ox;
                        let cy = (touches[0].client_y() + touches[1].client_y()) / 2.0 - oy;
                        state.update_pinch(dist, cx, cy);
                    }
                    _ => {}
                }
            },
            ontouchend: move |ev| {
                // `touches()` on touchend = remaining touches (finger just lifted excluded).
                let touches = ev.data().touches();
                match touches.len() {
                    0 => {
                        state.stop_drag();
                        state.stop_pan();
                        state.stop_pinch();
                    }
                    1 => {
                        // One finger lifted from a two-finger gesture — transition to pan.
                        state.stop_pinch();
                        if !locked { state.start_pan(touches[0].client_x(), touches[0].client_y()); }
                    }
                    _ => {}
                }
            },
            ontouchcancel: move |_| {
                state.stop_drag();
                state.stop_pan();
                state.stop_pinch();
            },

            // ── world (transformed) ──────────────────────────────────────────
            div {
                style: format!(
                    "position: absolute; top: 0; left: 0; width: 3000px; height: 2000px; transform: {transform}; transform-origin: 0 0;"
                ),

                div {
                    class: "absolute inset-0 pointer-events-none text-foreground",
                    style: "background-image: radial-gradient(circle, currentColor 1px, transparent 1px); background-size: 20px 20px; opacity: 0.12;",
                }

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
                    if let Some(preview) = state.connecting_preview() {
                        path {
                            d: preview.as_str(),
                            fill: "none",
                            stroke: "hsl(var(--primary))",
                            "stroke-width": "1.5",
                            "stroke-linecap": "round",
                            "stroke-dasharray": "5 4",
                            style: "opacity: 0.8;",
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
    children: Element,
) -> Element {
    // Guard against stale renders during deletion
    let node = { state.nodes.read().get(idx).cloned() };
    let Some(node) = node else { return rsx! {} };

    let (x, y)      = state.pos(idx);
    let is_active   = state.active_idx()   == Some(idx);
    let is_selected = state.selected_idx() == Some(idx);
    let is_connecting = state.is_connecting();
    let locked = state.is_locked();

    let width    = node.width;
    let node_id  = node.id.clone();
    let from_x   = x + width;
    let from_y   = y + NODE_H / 2.0;
    let to_id    = node.id.clone();
    let mut state = state;

    rsx! {
        div {
            class: format!(
                "absolute transition-shadow{}{}",
                if is_active   { " shadow-lg" } else { "" },
                if is_selected { " ring-2 ring-primary ring-offset-1 ring-offset-background rounded-md" } else { "" },
            ),
            style: format!("left: {x:.1}px; top: {y:.1}px; width: {width:.0}px; cursor: {};", if locked { "default" } else { "grab" }),

            onmousedown: move |ev| {
                ev.stop_propagation();
                if locked { return; }
                let c = ev.data().client_coordinates();
                state.select_node(idx);
                // handle's onmousedown fires first (inner→outer); if connecting already started, skip drag
                if !state.is_connecting() {
                    state.start_drag(idx, c.x, c.y);
                }
            },

            {children}

            // ── source handle (right) ─────────────────────────────────────
            if node.has_source {
                div {
                    class: format!(
                        "absolute right-0 top-1/2 size-3 rounded-full border-2 border-primary bg-background transition-colors{}",
                        if is_connecting { " ring-2 ring-primary/40 scale-125" } else { "" },
                    ),
                    style: "transform: translate(50%, -50%); cursor: crosshair; z-index: 10;",
                    "data-testid": "source-handle",
                    "data-node-id": "{node.id}",
                    onmousedown: move |ev| {
                        ev.stop_propagation();
                        if !locked { state.start_connect(node_id.clone(), from_x, from_y); }
                    },
                }
            }

            // ── target handle (left) ──────────────────────────────────────
            if node.has_target {
                div {
                    class: format!(
                        "absolute left-0 top-1/2 size-3 rounded-full border-2 border-primary bg-background transition-colors{}",
                        if is_connecting { " ring-2 ring-primary/40 scale-125" } else { "" },
                    ),
                    style: "transform: translate(-50%, -50%); cursor: crosshair; z-index: 10;",
                    "data-testid": "target-handle",
                    "data-node-id": "{node.id}",
                    onmouseup: move |ev| {
                        ev.stop_propagation();
                        state.finish_connect(to_id.clone());
                    },
                }
            }
        }
    }
}

// ── DefaultNodeContent ────────────────────────────────────────────────────────

#[component]
pub fn DefaultNodeContent(node: CanvasNode) -> Element {
    use crate::domain::test::components::node::{Node, NodeContent, NodeDescription, NodeHeader, NodeTitle};

    rsx! {
        Node {
            target: false,
            source: false,
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
        }
    }
}

// ── Minimap ───────────────────────────────────────────────────────────────────

#[component]
pub fn Minimap(state: NodeCanvasState) -> Element {
    let scale_x = MINI_W / WORLD_REF_W;
    let scale_y = MINI_H / WORLD_REF_H;
    let mut state = state;

    let pos_snap   = state.positions.read().clone();
    let nodes_snap = state.nodes.read().clone();
    let edges_snap = state.edges.read().clone();

    let node_rects: Vec<(f64, f64, f64, f64)> = nodes_snap
        .iter()
        .enumerate()
        .map(|(i, n)| {
            let (x, y) = pos_snap[i];
            (x * scale_x, y * scale_y, n.width * scale_x, NODE_H * scale_y)
        })
        .collect();

    let edge_paths: Vec<String> = edges_snap
        .iter()
        .filter_map(|edge| {
            let (fi, from) = nodes_snap.iter().enumerate().find(|(_, n)| n.id == edge.from)?;
            let (ti, _)    = nodes_snap.iter().enumerate().find(|(_, n)| n.id == edge.to)?;
            let (fx, fy) = pos_snap[fi];
            let (tx, ty) = pos_snap[ti];
            let sx  = (fx + from.width) * scale_x;
            let sy  = (fy + NODE_H / 2.0) * scale_y;
            let tx2 = tx * scale_x;
            let ty2 = (ty + NODE_H / 2.0) * scale_y;
            let dx  = (tx2 - sx).abs();
            let off = (dx / 2.0).clamp(4.0, 12.0);
            Some(format!(
                "M {sx:.1} {sy:.1} C {:.1} {sy:.1}, {:.1} {ty2:.1}, {tx2:.1} {ty2:.1}",
                sx + off,
                tx2 - off,
            ))
        })
        .collect();

    let (pan_x, pan_y) = *state.pan.read();
    let zoom = state.zoom_value();
    let vp_x = (-pan_x / zoom) * scale_x;
    let vp_y = (-pan_y / zoom) * scale_y;
    let vp_w = (VIEWPORT_W / zoom) * scale_x;
    let vp_h = (VIEWPORT_H / zoom) * scale_y;

    rsx! {
        div {
            class: "rounded-md border bg-background/90 backdrop-blur-sm shadow-sm overflow-hidden cursor-pointer",
            style: format!(
                "position: absolute; bottom: 12px; right: 12px; width: {MINI_W}px; height: {MINI_H}px;"
            ),
            // Click-to-pan: convert minimap element coords → world coords → center viewport there.
            onclick: move |ev| {
                let ec = ev.data().element_coordinates();
                let world_x = ec.x / scale_x;
                let world_y = ec.y / scale_y;
                let zoom = state.zoom_value();
                state.pan.set((
                    VIEWPORT_W / 2.0 - world_x * zoom,
                    VIEWPORT_H / 2.0 - world_y * zoom,
                ));
            },

            svg {
                width: "{MINI_W}",
                height: "{MINI_H}",

                for d in &edge_paths {
                    path {
                        d: d.as_str(),
                        fill: "none",
                        stroke: "hsl(var(--border))",
                        "stroke-width": "0.8",
                    }
                }

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

// ── Helpers ───────────────────────────────────────────────────────────────────

fn touch_dist(ax: f64, ay: f64, bx: f64, by: f64) -> f64 {
    let dx = ax - bx;
    let dy = ay - by;
    (dx * dx + dy * dy).sqrt()
}

// ── CanvasControls ────────────────────────────────────────────────────────────

#[component]
pub fn CanvasControls(state: NodeCanvasState) -> Element {
    let pct = (state.zoom_value() * 100.0).round() as i32;
    let mut state = state;

    rsx! {
        div {
            class: "flex items-center gap-0.5 rounded-md border bg-background/90 backdrop-blur-sm shadow-sm px-1.5 py-1",
            style: "position: absolute; top: 12px; right: 12px;",

            button {
                class: "size-6 flex items-center justify-center rounded text-sm hover:bg-accent transition-colors",
                title: "Add node",
                onclick: move |_| {
                    let (pan_x, pan_y) = *state.pan.read();
                    let zoom = state.zoom_value();
                    let cx = -pan_x / zoom + VIEWPORT_W / 2.0 / zoom;
                    let cy = -pan_y / zoom + VIEWPORT_H / 2.0 / zoom;
                    state.add_node(cx, cy);
                },
                "+"
            }

            div { class: "w-px h-4 bg-border mx-0.5" }

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
                onclick: move |_| { state.fit_to_view(VIEWPORT_W, VIEWPORT_H, NODE_H); },
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
