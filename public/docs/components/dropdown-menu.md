+++
title = "Dropdown Menu"
description = "Composable dropdown menu primitives for Dioxus."
tags = []
is_new = false
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++

<StaticDropdownMenu />

## Installation

<StaticInstallDropdownMenu />

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

<StaticDropdownMenu />

### Destructive Action

Menu item styled for destructive actions.

<StaticDropdownMenuDestructive />

### Start / End Alignment

Dropdown alignment examples relative to the trigger.

<StaticDropdownMenuStart />
<StaticDropdownMenuEnd />

### Outer Alignment

Examples positioned outside the trigger edge.

<StaticDropdownMenuStartOuter />
<StaticDropdownMenuEndOuter />

### User Menu

Profile-oriented dropdown patterns.

<StaticDropdownMenuUser />
<StaticDropdownMenuUserIcon />

### Radio And Select Patterns

Stateful dropdown choices for mutually exclusive options.

<StaticDropdownMenuRadio />
<StaticDropdownMenuSelect />

### RTL

Dropdown menu in right-to-left layout.

<StaticDropdownMenuRtl />

## See Also

- [Context Menu](/components/context_menu)
- [Menubar](/components/menubar)
