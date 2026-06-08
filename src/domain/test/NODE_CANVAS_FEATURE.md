# Node Canvas — Feature Tracker

## Already Implemented

| Feature                       | Notes                      |
|-------------------------------|----------------------------|
| Node drag + snap-to-grid      | 20px grid snap             |
| Node select / deselect        | click node, Escape to clear|
| Node delete                   | Del / Backspace on selected|
| Add node                      | double-click canvas        |
| Canvas pan                    | mouse drag on background   |
| Zoom                          | scroll wheel               |
| Zoom controls                 | +/− buttons, fit, reset    |
| Connect nodes                 | drag source → target handle|
| Bezier edges (animated dash)  | `edge-flow` CSS animation  |
| Connecting preview line       | live bezier to cursor      |
| Undo / redo                   | Ctrl+Z / Ctrl+Y            |
| Minimap overlay               | scaled world view          |
| Minimap click-to-pan          | click to center viewport   |
| Touch / trackpad support      | pinch-zoom + 1-finger pan  |
| Locked / read-only mode       | `toggle_locked()`, banner  |
| Multi-select                  | Shift+click, multi-drag    |

---

## Potential Features

| Feature                                                                       | Difficulty | Priority |
|-------------------------------------------------------------------------------|------------|----------|
| **Edge select + delete** — click edge to highlight, Del to remove             | Easy       | 🔴 High  |
| **Undo covers edge add/delete too** — history only tracks positions now       | Medium     | 🔴 High  |
| **Node inline edit** — double-click label to rename in place                  | Medium     | 🔴 High  |
| **Export / import JSON** — serialize `nodes + edges + positions`              | Medium     | 🔴 High  |
| **Animated edge direction arrow** — arrowhead at target end                   | Easy       | 🟠 Mid   |
| **Keyboard move selected node** — arrow keys nudge by grid step               | Easy       | 🟠 Mid   |
| **Node status badge** — running / error / done indicator on node              | Easy       | 🟠 Mid   |
| **Right-click context menu** — add/delete/duplicate at cursor                 | Medium     | 🟠 Mid   |
| ~~**Multi-select** — Shift+click to select many~~                             | ~~Medium~~ | ✅ Done  |
| **Copy / paste nodes** — Ctrl+C/V clones selection with offset                | Medium     | 🟠 Mid   |
| **Duplicate node** — Ctrl+D shortcut                                          | Medium     | 🟠 Mid   |
| **Auto-layout (dagre)** — compute acyclic layout from edges                   | Hard       | 🟠 Mid   |
| **Rubber-band multi-select** — drag empty canvas to draw selection rect       | Hard       | 🟠 Mid   |
| **Edge label** — text label rendered at bezier midpoint                       | Medium     | 🟡 Low   |
| **Alignment guides** — snap lines when dragging near other nodes              | Medium     | 🟡 Low   |
| **Edge style picker** — solid / dashed / dotted per edge                      | Medium     | 🟡 Low   |
| **Search / filter nodes** — highlight matching nodes, dim others              | Medium     | 🟡 Low   |
| **Node resize handle** — drag right edge to change `width`                    | Hard       | 🟡 Low   |
| **Orthogonal / step edge routing** — right-angle paths avoiding nodes         | Hard       | 🟡 Low   |
| **Node groups / subgraphs** — drag nodes into a named group container         | Hard       | 🟡 Low   |
