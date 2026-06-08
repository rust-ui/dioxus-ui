+++
title = "Select"
description = "Composable select component for choosing a single value in Dioxus."
tags = []
is_new = false
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++

<DemoSelect />

## Installation

Coming soon.

## Usage

```rust
use crate::ui::select::{Select, SelectContent, SelectGroup, SelectLabel, SelectOption, SelectTrigger, SelectValue};
```

```rust
rsx! {
    DemoSelect {}
}
```

## Examples

### Basic Select

Single-value select menu.

<DemoSelect />

### Scrollable

Scrollable select content for longer option lists.

<DemoSelectScrollable />

### RTL

Select menu in a right-to-left layout.

<DemoSelectRtl />

## See Also

- [Multi Select](/components/multi_select)
- [Select Native](/components/select_native)
