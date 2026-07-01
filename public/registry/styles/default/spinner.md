---
title: "Spinner"
name: "spinner"
cargo_dependencies: ["icons/dioxus", "tw_merge"]
registry_dependencies: []
type: "components:ui"
path: "ui/spinner.rs"
description: "A loading spinner component with animation for indicating processing states."
tags: []
---

# Spinner

A loading spinner component with animation for indicating processing states.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add spinner
```

## Component Code

```rust
use dioxus::prelude::*;
use icons::{Loader, LoaderCircle};
use tw_merge::tw_merge;

#[component]
pub fn Spinner(#[props(into, optional)] class: Option<String>) -> Element {
    let merged_class = tw_merge!("size-4 animate-spin", class.as_deref().unwrap_or(""));

    rsx! {
        LoaderCircle { class: "{merged_class}" }
    }
}

#[component]
pub fn SpinnerCircle(#[props(into, optional)] class: Option<String>) -> Element {
    let merged_class = tw_merge!("size-4 animate-spin", class.as_deref().unwrap_or(""));

    rsx! {
        Loader { class: "{merged_class}" }
    }
}
```
