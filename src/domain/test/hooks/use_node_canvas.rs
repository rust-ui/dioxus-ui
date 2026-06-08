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

#[derive(Clone, Copy, PartialEq)]
pub struct NodeCanvasState {
    pub positions: Signal<Vec<(f64, f64)>>,
    pub drag: Signal<Option<DragState>>,
}

impl NodeCanvasState {
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
            let dx = mx - d.mouse_start_x;
            let dy = my - d.mouse_start_y;
            self.positions.write()[d.node_idx] = (
                (d.node_start_x + dx).max(0.0),
                (d.node_start_y + dy).max(0.0),
            );
        }
    }

    pub fn stop_drag(&mut self) {
        self.drag.set(None);
    }

    pub fn edge_paths(&self, nodes: &[CanvasNode], edges: &[CanvasEdge], node_h: f64) -> Vec<String> {
        let pos = self.positions.read();
        edges
            .iter()
            .filter_map(|edge| {
                let (fi, from) = nodes.iter().enumerate().find(|(_, n)| n.id == edge.from)?;
                let (ti, _) = nodes.iter().enumerate().find(|(_, n)| n.id == edge.to)?;
                let (fx, fy) = pos[fi];
                let (tx, ty) = pos[ti];
                let sx = fx + from.width;
                let sy = fy + node_h / 2.0;
                let tex = tx;
                let tey = ty + node_h / 2.0;
                Some(bezier_path(sx, sy, tex, tey))
            })
            .collect()
    }
}

pub fn use_node_canvas(nodes: &[CanvasNode]) -> NodeCanvasState {
    let positions = use_signal(|| nodes.iter().map(|n| (n.initial_x, n.initial_y)).collect());
    let drag = use_signal(|| None);
    NodeCanvasState { positions, drag }
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
