+++
title = "Drawer"
description = "A bottom sheet that slides up from the bottom of the screen."
+++

<DemoDrawer />

## Installation

Coming soon.

## Usage

```rust
use crate::ui::drawer::{
    Drawer, DrawerTrigger, DrawerContent, DrawerHandle,
    DrawerHeader, DrawerTitle, DrawerDescription,
    DrawerBody, DrawerFooter, DrawerClose,
};
```

```rust
rsx! {
    Drawer {
        DrawerTrigger { "Open" }
        DrawerContent {
            DrawerHandle {}
            DrawerHeader {
                DrawerTitle { "Title" }
                DrawerDescription { "Description" }
            }
            DrawerBody { "Content here." }
            DrawerFooter {
                DrawerClose { "Cancel" }
            }
        }
    }
}
```

## See Also

- [Sheet](/components/sheet)
- [Dialog](/components/dialog)
