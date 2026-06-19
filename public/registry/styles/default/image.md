---
title: "Image"
name: "image"
cargo_dependencies: ["tw_merge"]
registry_dependencies: []
type: "components:ui"
path: "ui/image.rs"
description: "An image component with lazy loading and responsive support."
tags: []
---

# Image

An image component with lazy loading and responsive support.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add image
```

## Component Code

```rust
use dioxus::prelude::*;
use tw_merge::tw_merge;

#[component]
pub fn Image(
    #[props(into)] src: String,
    #[props(into)] alt: String,
    width: u32,
    height: u32,
    #[props(into, optional)] class: Option<String>,
    #[props(into, default = "lazy".to_string())] loading: String,
    #[props(into, default = "async".to_string())] decoding: String,
    #[props(into, optional)] srcset: Option<String>,
    #[props(into, optional)] sizes: Option<String>,
    #[props(default = false)] priority: bool,
) -> Element {
    let loading_attr = if priority { "eager".to_string() } else { loading };
    let merged = tw_merge!(class.as_deref().unwrap_or(""));
    rsx! {
        img {
            src: "{src}",
            alt: "{alt}",
            class: "{merged}",
            width: width,
            height: height,
            loading: "{loading_attr}",
            decoding: "{decoding}",
            srcset: srcset.as_deref().unwrap_or(""),
            sizes: sizes.as_deref().unwrap_or(""),
        }
    }
}
```
