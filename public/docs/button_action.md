+++
title = "Button Action"
description = "Press-and-hold action button for destructive or sensitive interactions in Dioxus."
tags = []
is_new = false
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++

<DemoButtonAction />

## Installation

Coming soon.

## Usage

```rust
use crate::ui::button_action::ButtonAction;
```

```rust
rsx! {
    ButtonAction { on_complete: move |_| {}, "Hold to confirm" }
}
```

## Examples

### Hold To Confirm

Requires the user to hold the button before the action completes. Useful for destructive actions where an accidental click would be too risky.

<DemoButtonAction />

## See Also

- [Button](/components/button)
- [Pressable](/components/pressable)
