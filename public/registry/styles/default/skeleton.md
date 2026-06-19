---
title: "Skeleton"
name: "skeleton"
cargo_dependencies: ["tw_merge"]
registry_dependencies: []
type: "components:ui"
path: "ui/skeleton.rs"
description: "Rust/UI component that show a placeholder while content is loading."
tags: []
---

# Skeleton

Rust/UI component that show a placeholder while content is loading.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add skeleton
```

## Component Code

```rust
use dioxus::prelude::*;
use tw_merge::tw_merge;

#[component]
pub fn Skeleton(#[props(into, optional)] class: Option<String>) -> Element {
    let merged_class = tw_merge!("animate-pulse rounded-md bg-muted", class.as_deref().unwrap_or(""));

    rsx! {
        div { class: "{merged_class}" }
    }
}
```
