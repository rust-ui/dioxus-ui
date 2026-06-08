+++
title = "Direction Provider"
description = "RTL and LTR direction context for Dioxus components."
tags = []
is_new = false
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++

<DemoDirectionProviderDefault />

## Installation

Coming soon.

## Usage

```rust
use crate::ui::direction_provider::DirectionProvider;
```

```rust
rsx! {
    DirectionProvider { dir: "rtl", "Your content" }
}
```

## Examples

### Default Direction

Default layout direction without overrides.

<DemoDirectionProviderDefault />

### Custom Direction

Explicit direction provider wrapping a subtree.

<DemoDirectionProvider />

### RTL

Right-to-left rendering for Arabic or Hebrew oriented interfaces.

<DemoDirectionProviderRtl />

## See Also

- [Button Group](/components/button_group)
- [Sheet](/components/sheet)
