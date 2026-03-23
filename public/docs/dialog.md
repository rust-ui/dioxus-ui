---
title: Dialog
description: A window overlaid on either the primary window or another dialog window, rendering content underneath inert.
tags: []
---

<DemoDialog />

## Installation

Add `dioxus_ui` to your `Cargo.toml`:

```toml
dioxus_ui = "0.1"
```

Also add `public/hooks/lock_scroll.js` to your `Dioxus.toml`:

```toml
[web.resource]
script = ["public/hooks/lock_scroll.js"]
```

## Usage

```rust
use dioxus_ui::dialog::{
    Dialog, DialogTrigger, DialogContent, DialogBody,
    DialogHeader, DialogTitle, DialogDescription,
    DialogFooter, DialogClose,
};
```

```rust
let mut open = use_signal(|| false);

rsx! {
    Dialog { open: open,
        DialogTrigger {
            onclick: move |_| open.set(true),
            "Open"
        }
        DialogContent { open: open,
            DialogBody {
                DialogHeader {
                    DialogTitle { "Edit profile" }
                    DialogDescription { "Make your changes below." }
                }
                // ... content ...
                DialogFooter {
                    DialogClose { open: open, "Cancel" }
                    Button { onclick: move |_| open.set(false), "Save" }
                }
            }
        }
    }
}
```

## Scrollable

<DemoDialogScrollable />

## See Also

- [Alert Dialog](/components/alert-dialog)
- [Collapsible](/components/collapsible)
