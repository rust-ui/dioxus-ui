---
title: "Use Lock Body Scroll"
name: "use_lock_body_scroll"
cargo_dependencies: []
registry_dependencies: []
type: "components:hooks"
path: "hooks/use_lock_body_scroll.rs"
description: "A Dioxus hook that locks and unlocks body scrolling, useful for modal dialogs, sheets, and overlays."
tags: []
---

# Use Lock Body Scroll

A Dioxus hook that locks and unlocks body scrolling, useful for modal dialogs, sheets, and overlays.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add use_lock_body_scroll
```

## Component Code

```rust
use dioxus::prelude::*;

pub fn use_lock_body_scroll(initial_locked: bool) -> Signal<bool> {
    let locked_signal = use_signal(|| initial_locked);

    use_effect(move || {
        if let Some(body) = web_sys::window().and_then(|w| w.document()).and_then(|d| d.body()) {
            let overflow = if locked_signal() { "hidden" } else { "" };
            let _ = body.style().set_property("overflow", overflow);
        }
    });

    locked_signal
}
```
