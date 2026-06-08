+++
title = "Combobox"
description = "Searchable selection UI built with command-style filtering in Dioxus."
tags = []
is_new = false
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++

<DemoCombobox />

## Installation

Coming soon.

## Usage

```rust
use crate::ui::command::{Command, CommandInput, CommandList, CommandItem};
use crate::ui::popover::{Popover, PopoverContent, PopoverTrigger};
```

```rust
rsx! {
    DemoCombobox {}
}
```

## Examples

### Searchable Select

Combines a popover with command-style filtering for fast keyboard-driven selection.

<DemoCombobox />

## See Also

- [Command](/components/command)
- [Select](/components/select)
