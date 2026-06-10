# Right-Click Context Menu — Implementation Plan

## Note on Registry Component

`registry/src/ui/context_menu.rs` is **Leptos**. Cannot import directly into the Dioxus workflow canvas.

Strategy: mirror the same data-attribute + vanilla JS pattern, but wire it through Dioxus signals for target tracking. The JS handles positioning/show/hide; Dioxus renders the correct items based on what was right-clicked.

---

## State Shape

### New enum + struct in `use_workflow.rs`

```rust
#[derive(Clone, PartialEq)]
pub enum ContextMenuTarget {
    Node(usize),   // node idx
    Edge(usize),   // edge idx (index into edges vec)
    Canvas,        // background
}

#[derive(Clone, PartialEq)]
pub struct ContextMenuState {
    pub target: ContextMenuTarget,
    pub x: f64,   // viewport x (clientX)
    pub y: f64,   // viewport y (clientY)
}
```

### Added to `WorkflowState`

```rust
pub context_menu: Signal<Option<ContextMenuState>>,
```

### New methods

```rust
pub fn open_context_menu(&mut self, target: ContextMenuTarget, x: f64, y: f64) {
    self.context_menu.set(Some(ContextMenuState { target, x, y }));
}

pub fn close_context_menu(&mut self) {
    self.context_menu.set(None);
}
```

---

## Trigger Points

| Right-click target | Where to add `oncontextmenu` | Target variant |
|--------------------|------------------------------|----------------|
| Node | `WorkflowNodeWrapper` div | `Node(idx)` |
| Edge SVG path | edge `<path>` element | `Edge(idx)` |
| Canvas background | canvas root div | `Canvas` |

All handlers call `e.prevent_default()` + `state.open_context_menu(target, e.client_x(), e.client_y())`.

---

## Menu Items per Target

```
Node right-click:
┌─────────────────┐
│ Rename          │  → state.start_edit(idx)   [if inline-edit implemented]
│ Duplicate       │  → state.duplicate_node(idx)
│ ─────────────── │
│ Delete          │  → state.delete_node(idx)  (red text)
└─────────────────┘

Edge right-click:
┌─────────────────┐
│ Delete Edge     │  → state.delete_edge(idx)  (red text)
└─────────────────┘

Canvas right-click:
┌─────────────────┐
│ Add Node Here   │  → state.add_node_at(canvas_x, canvas_y)
│ Select All      │  → state.select_all()
│ ─────────────── │
│ Paste           │  → state.paste_nodes()  (disabled if clipboard empty)
└─────────────────┘
```

---

## Rendering

Menu renders inside `WorkflowCanvas` overlay, outside the transform layer (fixed position, not affected by pan/zoom).

```
WorkflowCanvas
 ├── transform div (pan/zoom)
 │    ├── edges SVG
 │    └── nodes
 └── overlay slot
      ├── toolbar
      ├── controls
      └── ContextMenuOverlay  ← new, rendered when context_menu.is_some()
```

```rust
// ContextMenuOverlay component (in workflow.rs)
if let Some(cm) = state.context_menu.read().clone() {
    div {
        // fixed position, z-50
        style: "position: fixed; left: {cm.x}px; top: {cm.y}px; z-index: 50;",
        // viewport overflow correction via use_effect + web_sys rect check
        match cm.target {
            ContextMenuTarget::Node(idx) => rsx! { NodeContextMenu { state, idx } }
            ContextMenuTarget::Edge(idx) => rsx! { EdgeContextMenu { state, idx } }
            ContextMenuTarget::Canvas    => rsx! { CanvasContextMenu { state, cx: cm.x, cy: cm.y } }
        }
    }
}
```

### Close triggers

- Click outside menu → global `onclick` on canvas root with `stopPropagation` on menu itself
- `Escape` key → already handled in `use_workflow` keyboard handler, add `close_context_menu()` call
- Any action clicked → action handler calls `state.close_context_menu()` before executing

### Viewport overflow correction

After mount, read menu rect via `web_sys` and clamp `left`/`top` so menu stays within window. Same logic as registry JS `updatePosition()`, but done in a Dioxus `use_effect`.

---

## New helpers needed in `WorkflowState`

| Method | Purpose |
|--------|---------|
| `duplicate_node(idx)` | clone node with +20/+20 offset, new unique id |
| `delete_edge(idx)` | remove edge at index |
| `select_all()` | insert all node indices into `selected` |
| `add_node_at(canvas_x, canvas_y)` | add node at world coords (account for pan/zoom) |

`add_node_at` world coords from viewport:
```rust
let (px, py) = *self.pan.read();
let zoom = *self.zoom.read();
let wx = (viewport_x - px) / zoom;
let wy = (viewport_y - py) / zoom;
```

---

## Files to Change

| File | Change |
|------|--------|
| `hooks/use_workflow.rs` | `ContextMenuTarget`, `ContextMenuState`, field + methods + helpers |
| `components/workflow.rs` | `ContextMenuOverlay`, trigger handlers on node/edge/canvas |
| `demos/demo_workflow_context_menu.rs` | new demo |
| `demos/mod.rs` | `pub mod demo_workflow_context_menu` |

---

## Status

- [ ] `ContextMenuTarget` + `ContextMenuState` in `use_workflow.rs`
- [ ] `context_menu` field on `WorkflowState`
- [ ] `open_context_menu` / `close_context_menu` methods
- [ ] `duplicate_node`, `delete_edge`, `select_all`, `add_node_at` helpers
- [ ] `oncontextmenu` on node wrapper
- [ ] `oncontextmenu` on edge path
- [ ] `oncontextmenu` on canvas background
- [ ] `ContextMenuOverlay` rendered in canvas overlay
- [ ] Viewport overflow correction
- [ ] ESC closes menu
- [ ] Click-outside closes menu
- [ ] Demo component
- [ ] `cargo check` passes
