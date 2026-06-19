---
title: "Direction Provider"
name: "direction_provider"
cargo_dependencies: ["tw_merge"]
registry_dependencies: []
type: "components:ui"
path: "ui/direction_provider.rs"
description: "RTL and LTR direction context for Dioxus components."
tags: []
---

# Direction Provider

RTL and LTR direction context for Dioxus components.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add direction_provider
```

## Component Code

```rust
use dioxus::prelude::*;
use tw_merge::tw_merge;

#[derive(Default, Clone, Copy, PartialEq)]
pub enum Direction {
    #[default]
    Ltr,
    Rtl,
}

impl Direction {
    fn as_str(&self) -> &'static str {
        match self {
            Direction::Ltr => "ltr",
            Direction::Rtl => "rtl",
        }
    }
}

#[component]
pub fn DirectionProvider(
    #[props(default)] dir: Direction,
    #[props(into, optional)] class: Option<String>,
    children: Element,
) -> Element {
    let class = tw_merge!(class.as_deref().unwrap_or(""));
    rsx! {
        div {
            "data-slot": "direction-provider",
            dir: "{dir.as_str()}",
            class: "{class}",
            {children}
        }
    }
}
```
