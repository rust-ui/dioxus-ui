---
title: "Alert Dialog"
name: "alert_dialog"
cargo_dependencies: []
registry_dependencies: ["dialog"]
type: "components:ui"
path: "ui/alert_dialog.rs"
description: "This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality."
tags: []
---

# Alert Dialog

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add alert_dialog
```

## Component Code

```rust
use dioxus::prelude::*;

use crate::components::ui::dialog::{
    Dialog, DialogBody, DialogClose, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle,
    DialogTrigger,
};

#[component]
pub fn AlertDialog(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    rsx! { Dialog { class: class.unwrap_or_default(), {children} } }
}

#[component]
pub fn AlertDialogTrigger(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    rsx! { DialogTrigger { class: class.unwrap_or_default(), {children} } }
}

#[component]
pub fn AlertDialogContent(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    rsx! {
        DialogContent { class: class.unwrap_or_default(), close_on_backdrop_click: false, {children} }
    }
}

#[component]
pub fn AlertDialogBody(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    rsx! { DialogBody { class: class.unwrap_or_default(), {children} } }
}

#[component]
pub fn AlertDialogHeader(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    rsx! { DialogHeader { class: class.unwrap_or_default(), {children} } }
}

#[component]
pub fn AlertDialogTitle(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    rsx! { DialogTitle { class: class.unwrap_or_default(), {children} } }
}

#[component]
pub fn AlertDialogDescription(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    rsx! { DialogDescription { class: class.unwrap_or_default(), {children} } }
}

#[component]
pub fn AlertDialogFooter(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    rsx! { DialogFooter { class: class.unwrap_or_default(), {children} } }
}

#[component]
pub fn AlertDialogClose(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    rsx! { DialogClose { class: class.unwrap_or_default(), {children} } }
}

#[component]
pub fn AlertDialogAction(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    use crate::components::ui::dialog::DialogAction;
    rsx! { DialogAction { class: class.unwrap_or_default(), {children} } }
}
```
