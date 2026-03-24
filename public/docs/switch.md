---
title: "Switch"
name: "switch"
cargo_dependencies: []
registry_dependencies: []
type: "components:ui"
path: "ui/switch.rs"
description: "Rust/UI component that displays a control that allows the user to toggle between checked and not checked."
tags: ["utils"]
---

<DemoSwitch />

## Installation

Coming soon.

## Usage

```rust
use crate::ui::switch::Switch;
```

```rust
rsx! {
    Switch {}
}
```

## Examples

### With Label

Pair a switch with a label for a clear, accessible toggle control.

<DemoSwitchLabeled />

## See Also

- [Checkbox](/components/checkbox)
- [Label](/components/label)
