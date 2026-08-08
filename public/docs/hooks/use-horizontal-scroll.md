+++
title = "Use Horizontal Scroll"
description = "A Rust/UI hook that manages horizontal scrolling with state tracking and programmatic scroll controls."
tags = ["utils"]
is_new = false
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++

<StaticUseHorizontalScroll />

## Installation

<StaticInstallUseHorizontalScroll />

## Usage

```rust
use dioxus::prelude::*;
use registry::hooks::use_horizontal_scroll::{use_horizontal_scroll, HorizontalScrollState};
```

```rust
let container = use_signal(|| None::<web_sys::Element>);
let scroll_ctx = use_horizontal_scroll(container.into(), Some(0.75), Some(500));

scroll_ctx.scroll_by.call(-1);
scroll_ctx.scroll_by.call(1);

match scroll_ctx.scroll_state() {
    HorizontalScrollState::Start => {}
    HorizontalScrollState::Middle => {}
    HorizontalScrollState::End => {}
}
```

## Examples

### Default

<StaticUseHorizontalScroll />
