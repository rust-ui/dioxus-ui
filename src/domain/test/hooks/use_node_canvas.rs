use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct CanvasNode {
    pub id: &'static str,
    pub initial_x: f64,
    pub initial_y: f64,
    pub width: f64,
    pub has_target: bool,
    pub has_source: bool,
}

#[derive(Clone, PartialEq)]
pub struct CanvasEdge {
    pub from: &'static str,
    pub to: &'static str,
}

#[derive(Clone, Copy)]
pub struct DragState {
    pub node_idx: usize,
    pub mouse_start_x: f64,
    pub mouse_start_y: f64,
    pub node_start_x: f64,
    pub node_start_y: f64,
}

#[derive(Clone, Copy)]
struct PanState {
    mouse_start_x: f64,
    mouse_start_y: f64,
    pan_start_x: f64,
    pan_start_y: f64,
}

#[derive(Clone, Copy, PartialEq)]
pub struct NodeCanvasState {
    pub positions: Signal<Vec<(f64, f64)>>,
    pub drag: Signal<Option<DragState>>,
    pub pan: Signal<(f64, f64)>,
    pub zoom: Signal<f64>,
    canvas_drag: Signal<Option<PanState>>,
}

impl NodeCanvasState {
    // ── node drag ────────────────────────────────────────────────────────────

    pub fn pos(&self, idx: usize) -> (f64, f64) {
        self.positions.read()[idx]
    }

    pub fn is_dragging(&self) -> bool {
        self.drag.read().is_some()
    }

    pub fn active_idx(&self) -> Option<usize> {
        (*self.drag.read()).map(|d| d.node_idx)
    }

    pub fn start_drag(&mut self, idx: usize, mx: f64, my: f64) {
        let (nx, ny) = self.positions.read()[idx];
        self.drag.set(Some(DragState {
            node_idx: idx,
            mouse_start_x: mx,
            mouse_start_y: my,
            node_start_x: nx,
            node_start_y: ny,
        }));
    }

    pub fn update_drag(&mut self, mx: f64, my: f64) {
        if let Some(d) = *self.drag.read() {
            let z = *self.zoom.read();
            let dx = (mx - d.mouse_start_x) / z;
            let dy = (my - d.mouse_start_y) / z;
            self.positions.write()[d.node_idx] = (
                (d.node_start_x + dx).max(0.0),
                (d.node_start_y + dy).max(0.0),
            );
        }
    }

    pub fn stop_drag(&mut self) {
        self.drag.set(None);
    }

    // ── canvas pan ───────────────────────────────────────────────────────────

    pub fn is_panning(&self) -> bool {
        self.canvas_drag.read().is_some()
    }

    pub fn start_pan(&mut self, mx: f64, my: f64) {
        let (px, py) = *self.pan.read();
        self.canvas_drag.set(Some(PanState {
            mouse_start_x: mx,
            mouse_start_y: my,
            pan_start_x: px,
            pan_start_y: py,
        }));
    }

    pub fn update_pan(&mut self, mx: f64, my: f64) {
        if let Some(d) = *self.canvas_drag.read() {
            self.pan.set((
                d.pan_start_x + mx - d.mouse_start_x,
                d.pan_start_y + my - d.mouse_start_y,
            ));
        }
    }

    pub fn stop_pan(&mut self) {
        self.canvas_drag.set(None);
    }

    // ── zoom ─────────────────────────────────────────────────────────────────

    pub fn zoom_value(&self) -> f64 {
        *self.zoom.read()
    }

    /// Zoom anchored at cursor position (element-relative coords).
    pub fn zoom_at(&mut self, ex: f64, ey: f64, delta_y: f64) {
        let old_z = *self.zoom.read();
        let factor = if delta_y < 0.0 { 1.1_f64 } else { 1.0 / 1.1 };
        let new_z = (old_z * factor).clamp(0.2, 4.0);
        let ratio = new_z / old_z;
        let (px, py) = *self.pan.read();
        self.pan.set((ex - (ex - px) * ratio, ey - (ey - py) * ratio));
        self.zoom.set(new_z);
    }

    /// Zoom toward viewport center (used by +/- buttons).
    pub fn zoom_step(&mut self, factor: f64) {
        let old_z = *self.zoom.read();
        let new_z = (old_z * factor).clamp(0.2, 4.0);
        let ratio = new_z / old_z;
        let cx = 300.0_f64; // approximate half-width of canvas
        let cy = 225.0_f64; // approximate half-height (450px / 2)
        let (px, py) = *self.pan.read();
        self.pan.set((cx - (cx - px) * ratio, cy - (cy - py) * ratio));
        self.zoom.set(new_z);
    }

    pub fn zoom_reset(&mut self) {
        self.pan.set((0.0, 0.0));
        self.zoom.set(1.0);
    }

    pub fn world_transform(&self) -> String {
        let (px, py) = *self.pan.read();
        let z = *self.zoom.read();
        format!("translate({px:.2}px, {py:.2}px) scale({z:.4})")
    }

    // ── edges ────────────────────────────────────────────────────────────────

    pub fn edge_paths(&self, nodes: &[CanvasNode], edges: &[CanvasEdge], node_h: f64) -> Vec<String> {
        let pos = self.positions.read();
        edges
            .iter()
            .filter_map(|edge| {
                let (fi, from) = nodes.iter().enumerate().find(|(_, n)| n.id == edge.from)?;
                let (ti, _) = nodes.iter().enumerate().find(|(_, n)| n.id == edge.to)?;
                let (fx, fy) = pos[fi];
                let (tx, ty) = pos[ti];
                Some(bezier_path(
                    fx + from.width,
                    fy + node_h / 2.0,
                    tx,
                    ty + node_h / 2.0,
                ))
            })
            .collect()
    }
}

pub fn use_node_canvas(nodes: &[CanvasNode]) -> NodeCanvasState {
    let positions = use_signal(|| nodes.iter().map(|n| (n.initial_x, n.initial_y)).collect());
    NodeCanvasState {
        positions,
        drag: use_signal(|| None),
        pan: use_signal(|| (0.0, 0.0)),
        zoom: use_signal(|| 1.0),
        canvas_drag: use_signal(|| None),
    }
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
