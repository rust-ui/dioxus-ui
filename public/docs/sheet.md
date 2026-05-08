+++
title = "Sheet"
description = "A panel that slides in from any edge of the screen, built on top of Dialog."
+++

<DemoSheet />

## Installation

Coming soon.

## Usage

```rust
use crate::ui::sheet::{
    Sheet, SheetTrigger, SheetContent, SheetSide,
    SheetHeader, SheetTitle, SheetDescription,
    SheetBody, SheetFooter, SheetClose,
};
```

```rust
rsx! {
    Sheet {
        SheetTrigger { "Open" }
        SheetContent { side: SheetSide::Right,
            SheetHeader {
                SheetTitle { "Title" }
                SheetDescription { "Description" }
            }
            SheetBody { "Content here." }
            SheetFooter {
                SheetClose { "Cancel" }
            }
        }
    }
}
```

## See Also

- [Dialog](/components/dialog)
- [Drawer](/components/drawer)
