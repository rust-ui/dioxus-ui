use dioxus::prelude::*;

use super::use_history_stack::UseHistoryStack;

// ── NodeKind ──────────────────────────────────────────────────────────────────

#[derive(Clone, PartialEq)]
pub enum NodeKind {
    Trigger,
    Data,
    Agent,
    Output,
}

impl NodeKind {
    pub fn dot_color(&self) -> &'static str {
        match self {
            NodeKind::Trigger => "bg-yellow-500",
            NodeKind::Data    => "bg-blue-500",
            NodeKind::Agent   => "bg-purple-500",
            NodeKind::Output  => "bg-green-500",
        }
    }

    pub fn text_color(&self) -> &'static str {
        match self {
            NodeKind::Trigger => "text-yellow-600 dark:text-yellow-400",
            NodeKind::Data    => "text-blue-600 dark:text-blue-400",
            NodeKind::Agent   => "text-purple-600 dark:text-purple-400",
            NodeKind::Output  => "text-green-600 dark:text-green-400",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            NodeKind::Trigger => "Trigger",
            NodeKind::Data    => "Data",
            NodeKind::Agent   => "Agent",
            NodeKind::Output  => "Output",
        }
    }
}

// ── Data types ────────────────────────────────────────────────────────────────

#[derive(Clone, PartialEq)]
pub struct CanvasNode {
    pub id: String,
    pub initial_x: f64,
    pub initial_y: f64,
    pub width: f64,
    pub has_target: bool,
    pub has_source: bool,
    pub label: String,
    pub description: String,
    pub kind: NodeKind,
}

#[derive(Clone, PartialEq)]
pub struct CanvasEdge {
    pub from: String,
    pub to: String,
}

// ── ConnectingState ───────────────────────────────────────────────────────────

#[derive(Clone)]
pub struct ConnectingState {
    pub from_node_id: String,
    pub from_x: f64,
    pub from_y: f64,
    pub mouse_x: f64,
    pub mouse_y: f64,
}

// ── Internal drag/pan state ───────────────────────────────────────────────────

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

// ── NodeCanvasState ───────────────────────────────────────────────────────────

pub struct NodeCanvasState {
    pub nodes:      Signal<Vec<CanvasNode>>,
    pub edges:      Signal<Vec<CanvasEdge>>,
    pub positions:  Signal<Vec<(f64, f64)>>,
    pub drag:       Signal<Option<DragState>>,
    pub pan:        Signal<(f64, f64)>,
    pub zoom:       Signal<f64>,
    pub selected:   Signal<Option<usize>>,
    pub connecting: Signal<Option<ConnectingState>>,
    canvas_drag:    Signal<Option<PanState>>,
    pub history:    UseHistoryStack<Vec<(f64, f64)>>,
    next_id:        Signal<usize>,
}

// Manual Copy/Clone/PartialEq — Signal<Vec<T>> is Copy regardless of T.
impl Copy for NodeCanvasState {}
impl Clone for NodeCanvasState {
    fn clone(&self) -> Self { *self }
}
impl PartialEq for NodeCanvasState {
    fn eq(&self, other: &Self) -> bool {
        self.nodes      == other.nodes
            && self.edges     == other.edges
            && self.positions == other.positions
            && self.drag      == other.drag
            && self.pan       == other.pan
            && self.zoom      == other.zoom
            && self.selected  == other.selected
            && self.connecting == other.connecting
            && self.history   == other.history
    }
}

impl NodeCanvasState {
    // ── selection ────────────────────────────────────────────────────────────

    pub fn selected_idx(&self) -> Option<usize> {
        *self.selected.read()
    }

    pub fn select_node(&mut self, idx: usize) {
        self.selected.set(Some(idx));
    }

    pub fn deselect(&mut self) {
        self.selected.set(None);
    }

    // ── connect ───────────────────────────────────────────────────────────────

    pub fn is_connecting(&self) -> bool {
        self.connecting.read().is_some()
    }

    pub fn start_connect(&mut self, from_node_id: String, from_x: f64, from_y: f64) {
        self.connecting.set(Some(ConnectingState {
            from_node_id,
            from_x, from_y,
            mouse_x: from_x, mouse_y: from_y,
        }));
    }

    pub fn update_connect_mouse(&mut self, ex: f64, ey: f64) {
        if self.connecting.read().is_none() { return; }
        let (pan_x, pan_y) = *self.pan.read();
        let zoom = *self.zoom.read();
        let wx = (ex - pan_x) / zoom;
        let wy = (ey - pan_y) / zoom;
        if let Some(cs) = self.connecting.write().as_mut() {
            cs.mouse_x = wx;
            cs.mouse_y = wy;
        }
    }

    pub fn finish_connect(&mut self, to_node_id: String) {
        let cs = self.connecting.read().clone();
        self.connecting.set(None);
        let Some(cs) = cs else { return };
        if cs.from_node_id == to_node_id { return; }
        let already = self.edges.read().iter()
            .any(|e| e.from == cs.from_node_id && e.to == to_node_id);
        if !already {
            self.edges.write().push(CanvasEdge { from: cs.from_node_id, to: to_node_id });
        }
    }

    pub fn cancel_connect(&mut self) {
        self.connecting.set(None);
    }

    pub fn connecting_preview(&self) -> Option<String> {
        let cs = self.connecting.read().clone()?;
        Some(bezier_path(cs.from_x, cs.from_y, cs.mouse_x, cs.mouse_y))
    }

    // ── delete ───────────────────────────────────────────────────────────────

    pub fn delete_selected(&mut self) {
        let Some(idx) = *self.selected.read() else { return };
        let id = self.nodes.read()[idx].id.clone();
        self.nodes.write().remove(idx);
        self.positions.write().remove(idx);
        self.edges.write().retain(|e| e.from != id && e.to != id);
        self.selected.set(None);
        let snap = self.positions.read().clone();
        self.history.push(snap);
    }

    // ── add node ─────────────────────────────────────────────────────────────

    pub fn add_node(&mut self, x: f64, y: f64) {
        let n = *self.next_id.read();
        *self.next_id.write() = n + 1;
        let snap_x = (x / 20.0).round() * 20.0;
        let snap_y = (y / 20.0).round() * 20.0;
        // positions must be pushed before nodes: writing nodes triggers a re-render,
        // and the render loop indexes into positions by node idx — if positions is
        // shorter than nodes at that moment, pos() panics with index out of bounds.
        self.positions.write().push((snap_x, snap_y));
        self.nodes.write().push(CanvasNode {
            id: format!("node-{n}"),
            initial_x: snap_x,
            initial_y: snap_y,
            width: 192.0,
            has_target: true,
            has_source: true,
            label: format!("Node {n}"),
            description: "New node".to_string(),
            kind: NodeKind::Agent,
        });
    }

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
            let raw_x = (d.node_start_x + dx).max(0.0);
            let raw_y = (d.node_start_y + dy).max(0.0);
            self.positions.write()[d.node_idx] = (
                (raw_x / 20.0).round() * 20.0,
                (raw_y / 20.0).round() * 20.0,
            );
        }
    }

    pub fn stop_drag(&mut self) {
        if self.drag.read().is_some() {
            let snap = self.positions.read().clone();
            self.history.push(snap);
        }
        self.drag.set(None);
    }

    pub fn undo(&mut self) {
        if let Some(snap) = self.history.undo() {
            *self.positions.write() = snap;
        }
    }

    pub fn redo(&mut self) {
        if let Some(snap) = self.history.redo() {
            *self.positions.write() = snap;
        }
    }

    pub fn can_undo(&self) -> bool { self.history.can_undo() }
    pub fn can_redo(&self) -> bool { self.history.can_redo() }

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

    pub fn zoom_value(&self) -> f64 { *self.zoom.read() }

    pub fn zoom_at(&mut self, ex: f64, ey: f64, delta_y: f64) {
        let old_z = *self.zoom.read();
        let factor = if delta_y < 0.0 { 1.1_f64 } else { 1.0 / 1.1 };
        let new_z = (old_z * factor).clamp(0.2, 4.0);
        let ratio = new_z / old_z;
        let (px, py) = *self.pan.read();
        self.pan.set((ex - (ex - px) * ratio, ey - (ey - py) * ratio));
        self.zoom.set(new_z);
    }

    pub fn zoom_step(&mut self, factor: f64) {
        let old_z = *self.zoom.read();
        let new_z = (old_z * factor).clamp(0.2, 4.0);
        let ratio = new_z / old_z;
        let cx = 400.0_f64;
        let cy = 225.0_f64;
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

    // ── fit to view ───────────────────────────────────────────────────────────

    pub fn fit_to_view(&mut self, viewport_w: f64, viewport_h: f64, node_h: f64) {
        let nodes = self.nodes.read();
        if nodes.is_empty() { return; }
        let padding = 48.0;
        let pos = self.positions.read();
        let min_x = nodes.iter().enumerate().map(|(i, _)| pos[i].0).fold(f64::INFINITY, f64::min);
        let min_y = nodes.iter().enumerate().map(|(i, _)| pos[i].1).fold(f64::INFINITY, f64::min);
        let max_x = nodes.iter().enumerate().map(|(i, n)| pos[i].0 + n.width).fold(f64::NEG_INFINITY, f64::max);
        let max_y = nodes.iter().enumerate().map(|(i, _)| pos[i].1 + node_h).fold(f64::NEG_INFINITY, f64::max);
        drop(pos);
        drop(nodes);
        let content_w = (max_x - min_x).max(1.0);
        let content_h = (max_y - min_y).max(1.0);
        let z = ((viewport_w - padding * 2.0) / content_w)
            .min((viewport_h - padding * 2.0) / content_h)
            .clamp(0.2, 4.0);
        self.zoom.set(z);
        self.pan.set((
            (viewport_w - content_w * z) / 2.0 - min_x * z,
            (viewport_h - content_h * z) / 2.0 - min_y * z,
        ));
    }

    // ── edges ────────────────────────────────────────────────────────────────

    pub fn edge_paths(&self, node_h: f64) -> Vec<String> {
        let pos   = self.positions.read();
        let nodes = self.nodes.read();
        let edges = self.edges.read();
        edges
            .iter()
            .filter_map(|edge| {
                let (fi, from) = nodes.iter().enumerate().find(|(_, n)| n.id == edge.from)?;
                let (ti, _)    = nodes.iter().enumerate().find(|(_, n)| n.id == edge.to)?;
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

// ── Constructor ───────────────────────────────────────────────────────────────

pub fn use_node_canvas(nodes: Vec<CanvasNode>, edges: Vec<CanvasEdge>) -> NodeCanvasState {
    let initial: Vec<(f64, f64)> = nodes.iter().map(|n| (n.initial_x, n.initial_y)).collect();
    let next_id = nodes.len();
    NodeCanvasState {
        nodes:      use_signal(|| nodes),
        edges:      use_signal(|| edges),
        positions:  use_signal(|| initial.clone()),
        drag:       use_signal(|| None),
        pan:        use_signal(|| (0.0, 0.0)),
        zoom:       use_signal(|| 1.0),
        selected:   use_signal(|| None),
        connecting: use_signal(|| None),
        canvas_drag: use_signal(|| None),
        history:    UseHistoryStack::new(initial),
        next_id:    use_signal(|| next_id),
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn bezier_path(sx: f64, sy: f64, tx: f64, ty: f64) -> String {
    let dx = (tx - sx).abs();
    let offset = (dx / 2.0).clamp(40.0, 80.0);
    format!(
        "M {sx:.1} {sy:.1} C {:.1} {sy:.1}, {:.1} {ty:.1}, {tx:.1} {ty:.1}",
        sx + offset,
        tx - offset,
    )
}
