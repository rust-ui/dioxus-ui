---
title: "Separator"
name: "separator"
cargo_dependencies: ["tw_merge"]
registry_dependencies: []
type: "components:ui"
path: "ui/separator.rs"
description: "Rust/UI component that displays a separator line."
tags: []
---

# Separator

Rust/UI component that displays a separator line.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add separator
```

## Component Code

```rust
use dioxus::prelude::*;
use tw_merge::tw_merge;

#[allow(dead_code)]
#[derive(Default, Clone, PartialEq)]
pub enum SeparatorOrientation {
    #[default]
    Horizontal,
    Vertical,
}

#[component]
pub fn Separator(
    #[props(into, optional)] class: Option<String>,
    #[props(default = SeparatorOrientation::default())] orientation: SeparatorOrientation,
) -> Element {
    let orientation_class = match orientation {
        SeparatorOrientation::Horizontal => "w-full h-[1px]",
        SeparatorOrientation::Vertical => "h-full w-[1px]",
    };
    let merged_class = tw_merge!("shrink-0 bg-border", orientation_class, class.as_deref().unwrap_or(""));

    rsx! {
        div { class: "{merged_class}", role: "separator" }
    }
}
```
