+++
title = "Context Menu"
description = "Right-click context menu patterns for Dioxus applications."
tags = []
is_new = false
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++

<DemoContextMenu />

## Installation

Coming soon.

## Usage

```rust
use crate::ui::context_menu::{ContextMenu, ContextMenuContent, ContextMenuItem, ContextMenuTrigger};
```

```rust
rsx! {
    DemoContextMenu {}
}
```

## Examples

### Basic Context Menu

Right-click menu for quick actions on a target element.

<DemoContextMenu />

### Action Menu

Context menu including an action that requires an additional hold-to-confirm step.

<DemoContextMenuAction />

### RTL

Context menu rendered in a right-to-left layout.

<DemoContextMenuRtl />

## See Also

- [Dropdown Menu](/components/dropdown_menu)
- [Command](/components/command)
