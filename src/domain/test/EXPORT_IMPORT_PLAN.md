# Export / Import JSON — Implementation Plan

## Goal

Serialize the full canvas state (`nodes + edges + positions + viewport`) to JSON and restore it.

---

## Data Shape

```rust
// New struct in use_workflow.rs
#[derive(Serialize, Deserialize)]
pub struct WorkflowSnapshot {
    pub nodes: Vec<WorkflowNode>,
    pub edges: Vec<WorkflowEdge>,
    pub positions: Vec<(f64, f64)>,
    pub pan: (f64, f64),
    pub zoom: f64,
}
```

---

## Files to Change

| File | Change |
|------|--------|
| `Cargo.toml` | add `serde_json = "1"` |
| `src/domain/test/hooks/use_workflow.rs` | serde derives + `WorkflowSnapshot` + 2 methods |
| `src/domain/test/demos/demo_workflow_export_import.rs` | new demo |
| `src/domain/test/demos/mod.rs` | `pub mod demo_workflow_export_import` |

---

## Changes in `use_workflow.rs`

### 1. Add serde derives

```rust
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum WorkflowNodeKind { Trigger, Data, Agent, Output }

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkflowNode { /* unchanged fields */ }

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkflowEdge { pub from: String, pub to: String }
```

### 2. New `WorkflowSnapshot` struct

Place after `WorkflowEdge` definition.

### 3. Two new methods on `WorkflowState`

```rust
pub fn export_snapshot(&self) -> WorkflowSnapshot {
    WorkflowSnapshot {
        nodes: self.nodes.read().clone(),
        edges: self.edges.read().clone(),
        positions: self.positions.read().clone(),
        pan: *self.pan.read(),
        zoom: *self.zoom.read(),
    }
}

pub fn load_snapshot(&mut self, snap: WorkflowSnapshot) {
    let next_id = snap.nodes.len();
    // positions BEFORE nodes — render loop indexes positions by node idx
    self.positions.set(snap.positions);
    self.nodes.set(snap.nodes);
    self.edges.set(snap.edges);
    self.pan.set(snap.pan);
    self.zoom.set(snap.zoom);
    self.next_id.set(next_id);
    self.selected.write().clear();
    self.drag.set(None);
    self.connecting.set(None);
}
```

---

## Demo UI

```
┌──────────────────────────────────────────────────────┐
│  [↓ Export JSON]  [↑ Import JSON]     (top toolbar)  │
│                                                      │
│   ┌────────┐        ┌──────────┐                     │
│   │ Input  │───────▶│ Processor│                     │
│   └────────┘   ┌───▶└──────────┘                     │
│                │    ┌──────────┐   ┌────────┐        │
│                └───▶│Validator │──▶│ Output │        │
│                     └──────────┘   └────────┘        │
│                                  [+][−][⊡] (controls)│
└──────────────────────────────────────────────────────┘
```

### Export flow
1. Call `state.export_snapshot()`
2. `serde_json::to_string_pretty(&snap)`
3. JS `eval()` triggers blob download:
```js
const blob = new Blob([dioxus.recv()], {type:'application/json'});
const url = URL.createObjectURL(blob);
const a = document.createElement('a');
a.href = url; a.download = 'workflow.json'; a.click();
URL.revokeObjectURL(url);
```

### Import flow
1. Hidden `<input type="file" accept=".json">`
2. Dioxus `onchange` → `FileEngine::read_file_to_string` (async)
3. `serde_json::from_str::<WorkflowSnapshot>(&content)`
4. On success: `state.load_snapshot(snap)`
5. On error: show toast / log

---

## Key Invariant

`positions` must be written **before** `nodes` in `load_snapshot`.  
Render loop indexes `positions[idx]` immediately after writing `nodes` triggers re-render.  
Breaking this order = index-out-of-bounds panic. See `use_workflow.rs:376-379`.

---

## Status

- [ ] `serde_json` in Cargo.toml
- [ ] serde derives on `WorkflowNodeKind`, `WorkflowNode`, `WorkflowEdge`
- [ ] `WorkflowSnapshot` struct
- [ ] `export_snapshot` method
- [ ] `load_snapshot` method
- [ ] `demo_workflow_export_import.rs`
- [ ] `mod.rs` updated
- [ ] `cargo check` passes
