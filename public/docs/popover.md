+++
title = "Popover"
description = "A floating panel that opens on click, anchored to a trigger element."
+++

<DemoPopover />

## Installation

Coming soon.

## Usage

```rust
use crate::ui::popover::{Popover, PopoverTrigger, PopoverContent, PopoverSide};
```

```rust
rsx! {
    Popover {
        PopoverTrigger { "Open" }
        PopoverContent {
            p { "Popover content here." }
        }
    }
}
```

## See Also

- [Hover Card](/components/hover-card)
- [Tooltip](/components/tooltip)
- [Dialog](/components/dialog)
