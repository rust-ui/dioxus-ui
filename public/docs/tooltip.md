---
title: "Tooltip"
name: "tooltip"
cargo_dependencies: []
registry_dependencies: []
type: "components:ui"
path: "ui/tooltip.rs"
description: "A popup component that displays informative text when users hover over, focus on, or click an element."
tags: ["dialog"]
---

<DemoTooltip />

## Installation

Coming soon.

## Usage

```rust
use crate::ui::tooltip::{Tooltip, TooltipContent, TooltipPosition};
```

```rust
rsx! {
    Tooltip {
        Button { "Hover me" }
        TooltipContent { "This is a tooltip" }
    }
}
```

## Positions

The tooltip can be positioned on any side: `Top` (default), `Bottom`, `Left`, `Right`.

```rust
rsx! {
    Tooltip {
        Button { "Right side" }
        TooltipContent {
            position: TooltipPosition::Right,
            "Tooltip on the right"
        }
    }
}
```

## See Also

- [Badge](/components/badge)
- [Popover](/components/popover)
