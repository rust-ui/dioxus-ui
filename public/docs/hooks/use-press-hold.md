+++
title = "Use Press Hold"
description = "A Rust/UI hook for press-and-hold interactions with animated progress."
tags = ["utils"]
is_new = false
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++

<StaticUsePressHold />

## Installation

<StaticInstallUsePressHold />

## Usage

```rust
use dioxus::prelude::*;
use registry::hooks::use_press_hold::use_press_hold;
```

```rust
let on_complete = Callback::new(move |_| {
    println!("Completed");
});

let press_hold = use_press_hold(2000, on_complete, false);

rsx! {
    button {
        onpointerdown: move |_| press_hold.on_pointer_down(),
        onpointerup: move |_| press_hold.on_pointer_up(),
        onpointerleave: move |_| press_hold.on_pointer_up(),
        onpointercancel: move |_| press_hold.on_pointer_up(),
        "{format!(\"{:.0}%\", press_hold.progress_signal() * 100.0)}"
    }
}
```

## Examples

### Default

<StaticUsePressHold />
