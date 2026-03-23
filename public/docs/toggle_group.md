---
title: Toggle Group
description: A set of two-state buttons that can be toggled on or off, grouped together.
tags: []
---

<DemoToggleGroup />

## Installation

Add `dioxus_ui` to your `Cargo.toml`:

```toml
dioxus_ui = "0.1"
```

## Usage

```rust
use dioxus_ui::toggle_group::{ToggleGroup, ToggleGroupItem};
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
