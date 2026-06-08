+++
title = "Dropdown Menu"
description = "Composable dropdown menu primitives for Dioxus."
tags = []
is_new = false
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++

<DemoDropdownMenu />

## Installation

Coming soon.

## Usage

```rust
use crate::ui::dropdown_menu::{
    DropdownMenu,
    DropdownMenuContent,
    DropdownMenuItem,
    DropdownMenuTrigger,
};
```

```rust
rsx! {
    DemoDropdownMenu {}
}
```

## Examples

### Basic Menu

Default dropdown menu with a trigger and action items.

<DemoDropdownMenu />

### Destructive Action

Menu item styled for destructive actions.

<DemoDropdownMenuDestructive />

### Start / End Alignment

Dropdown alignment examples relative to the trigger.

<DemoDropdownMenuStart />
<DemoDropdownMenuEnd />

### Outer Alignment

Examples positioned outside the trigger edge.

<DemoDropdownMenuStartOuter />
<DemoDropdownMenuEndOuter />

### User Menu

Profile-oriented dropdown patterns.

<DemoDropdownMenuUser />
<DemoDropdownMenuUserIcon />

### Radio And Select Patterns

Stateful dropdown choices for mutually exclusive options.

<DemoDropdownMenuRadio />
<DemoDropdownMenuSelect />

### RTL

Dropdown menu in right-to-left layout.

<DemoDropdownMenuRtl />

## See Also

- [Context Menu](/components/context_menu)
- [Menubar](/components/menubar)
