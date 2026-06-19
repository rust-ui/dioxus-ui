---
title: "Pressable"
name: "pressable"
cargo_dependencies: ["tw_merge"]
registry_dependencies: []
type: "components:ui"
path: "ui/pressable.rs"
description: "A wrapper that adds press feedback (scale effect) to any children."
tags: []
---

# Pressable

A wrapper that adds press feedback (scale effect) to any children.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add pressable
```

## Component Code

```rust
use dioxus::prelude::*;
use tw_merge::tw_merge;

#[component]
pub fn Pressable(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let mut is_pressed = use_signal(|| false);
    let base = tw_merge!("transition-transform", class.as_deref().unwrap_or(""));
    let class_str = use_memo(move || if is_pressed() { format!("{base} scale-[0.98]") } else { base.clone() });

    rsx! {
        div {
            class: "{class_str}",
            onpointerdown: move |_| is_pressed.set(true),
            onpointerup: move |_| is_pressed.set(false),
            onpointerleave: move |_| is_pressed.set(false),
            onpointercancel: move |_| is_pressed.set(false),
            {children}
        }
    }
}
```
