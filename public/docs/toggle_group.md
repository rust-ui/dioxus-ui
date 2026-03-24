---
title: "Toggle Group"
name: "toggle_group"
cargo_dependencies: []
registry_dependencies: []
type: "components:ui"
path: "ui/toggle_group.rs"
---

<DemoToggleGroup />

## Installation

Coming soon.

## Usage

```rust
use crate::ui::toggle_group::{ToggleGroup, ToggleGroupItem};
```

```rust
let mut selected = use_signal(|| "center".to_string());

rsx! {
    ToggleGroup {
        ToggleGroupItem {
            value: "left",
            pressed: selected() == "left",
            onclick: move |_| selected.set("left".to_string()),
            "Left"
        }
        ToggleGroupItem {
            value: "center",
            pressed: selected() == "center",
            onclick: move |_| selected.set("center".to_string()),
            "Center"
        }
        ToggleGroupItem {
            value: "right",
            pressed: selected() == "right",
            onclick: move |_| selected.set("right".to_string()),
            "Right"
        }
    }
}
```

## See Also

- [Toggle](/components/toggle)
- [Tabs](/components/tabs)
