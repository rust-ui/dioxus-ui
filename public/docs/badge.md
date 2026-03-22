---
title: Badge
description: Displays a badge or a component that looks like a badge.
---

## Demo

<DemoBadge />

## Variants

<DemoBadgeVariants />

## Colors

<DemoBadgeColors />

## Custom

<DemoBadgeCustom />

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
