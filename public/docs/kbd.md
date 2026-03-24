---
title: "Kbd"
name: "kbd"
cargo_dependencies: []
registry_dependencies: []
type: "components:ui"
path: "ui/kbd.rs"
description: "Display keyboard shortcuts and key combinations with proper styling."
tags: ["utils"]
---

<DemoKbd />

## Installation

Coming soon.

## Usage

```rust
use crate::ui::kbd::Kbd;
```

```rust
rsx! {
    Kbd { "⌘K" }
}
```

## Examples

### Key Combination

Combine multiple `Kbd` elements with a separator to represent multi-key shortcuts.

## See Also

- [Badge](/components/badge)
- [Button](/components/button)
