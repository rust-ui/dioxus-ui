---
title: Badge
description: Displays a badge or a component that looks like a badge.
---

## Demo

<demo-badge></demo-badge>

## Variants

<demo-badge-variants></demo-badge-variants>

## Colors

<demo-badge-colors></demo-badge-colors>

## Custom

<demo-badge-custom></demo-badge-custom>

## Props

| Prop      | Type             | Default   | Description               |
|-----------|------------------|-----------|---------------------------|
| `variant` | `BadgeVariant`   | `default` | Visual style              |
| `class`   | `Option<String>` | `None`    | Extra Tailwind classes    |

## Usage

```rust
rsx! {
    Badge { "New" }
    Badge { variant: BadgeVariant::Destructive, "Error" }
}
```
