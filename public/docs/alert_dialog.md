---
title: Alert Dialog
description: A modal dialog that interrupts the user with important content and expects a response.
tags: []
---

<DemoAlertDialog />

## Installation

Coming soon.

## Usage

```rust
use crate::ui::alert_dialog::{
    AlertDialog, AlertDialogHeader, AlertDialogTitle,
    AlertDialogDescription, AlertDialogFooter,
};
```

```rust
let mut open = use_signal(|| false);

rsx! {
    Button {
        onclick: move |_| open.set(true),
        "Open Dialog"
    }
    AlertDialog {
        open: open,
        AlertDialogHeader {
            AlertDialogTitle { "Are you sure?" }
            AlertDialogDescription { "This action cannot be undone." }
        }
        AlertDialogFooter {
            Button {
                variant: ButtonVariant::Outline,
                onclick: move |_| open.set(false),
                "Cancel"
            }
            Button {
                variant: ButtonVariant::Destructive,
                onclick: move |_| open.set(false),
                "Continue"
            }
        }
    }
}
```

## See Also

- [Alert](/components/alert)
- [Button](/components/button)
