---
title: Toggle
description: A two-state button that can be either on or off.
tags: []
---

<DemoToggle />

## Installation

Add `dioxus_ui` to your `Cargo.toml`:

```toml
dioxus_ui = "0.1"
```

## Usage

```rust
use dioxus_ui::toggle::{Toggle, ToggleVariant};
```

```rust
let mut pressed = use_signal(|| false);

rsx! {
    Toggle {
        pressed: pressed(),
        onclick: move |_| pressed.toggle(),
        "Bold"
    }
}
```

## Variants

- `Default` — subtle background on active
- `Outline` — bordered variant

## See Also

- [Button](/components/button)
- [Switch](/components/switch)
