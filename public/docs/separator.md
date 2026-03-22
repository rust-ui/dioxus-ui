---
title: Separator
description: Visually or semantically separates content.
---

## Demo

<DemoSeparator />

## Props

| Prop          | Type              | Default        | Description             |
|---------------|-------------------|----------------|-------------------------|
| `orientation` | `Orientation`     | `horizontal`   | Horizontal or vertical  |
| `class`       | `Option<String>`  | `None`         | Extra Tailwind classes  |

## Usage

```rust
rsx! {
    Separator {}
    Separator { orientation: Orientation::Vertical }
}
```
