---
title: "Aspect Ratio"
name: "aspect_ratio"
cargo_dependencies: ["tw_merge"]
registry_dependencies: []
type: "components:ui"
path: "ui/aspect_ratio.rs"
description: "A container that maintains a given aspect ratio for its content."
tags: []
---

# Aspect Ratio

A container that maintains a given aspect ratio for its content.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add aspect_ratio
```

## Component Code

```rust
use dioxus::prelude::*;
use tw_merge::tw_merge;

#[component]
pub fn AspectRatio(
    #[props(default = 1.7777777777777777_f64)] ratio: f64,
    #[props(into, optional)] class: Option<String>,
    children: Element,
) -> Element {
    let style = format!("aspect-ratio: {ratio}");
    let merged = tw_merge!("relative w-full overflow-hidden", class.as_deref().unwrap_or(""));
    rsx! {
        div { "data-name": "AspectRatio", class: "{merged}", style: "{style}", {children} }
    }
}
```
