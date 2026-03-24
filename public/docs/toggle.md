---
title: "Toggle"
name: "toggle"
cargo_dependencies: []
registry_dependencies: []
type: "components:ui"
path: "ui/toggle.rs"
description: "Rust/UI component that displays a two-state button that can be either on or off."
tags: ["form"]
---

<DemoToggle />

## Installation

Coming soon.

## Usage

```rust
use crate::ui::toggle::Toggle;
```

```rust
rsx! {
    Toggle {
        pressed: pressed(),
        onclick: move |_| pressed.set(!pressed()),
        "Bold"
    }
}
```

## See Also

- [Switch](/components/switch)
- [Toggle Group](/components/toggle-group)
