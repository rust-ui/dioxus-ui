---
title: Tooltip
description: A popup that displays information related to an element when the element receives keyboard focus or the mouse hovers over it.
tags: []
---

<DemoTooltip />

## Installation

Add `dioxus_ui` to your `Cargo.toml`:

```toml
dioxus_ui = "0.1"
```

## Usage

```rust
use dioxus_ui::tooltip::{Tooltip, TooltipContent, TooltipPosition};
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
