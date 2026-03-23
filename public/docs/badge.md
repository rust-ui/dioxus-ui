---
title: Badge
description: Dioxus UI component that displays a badge or a component that looks like a badge.
tags: ["badge"]
---

<DemoBadge />

## Installation

Coming soon.

## Usage

```rust
use crate::ui::badge::Badge;
```

```rust
rsx! {
    Badge { "Badge" }
}
```

## Examples

### Variants

Available Badge style variants include default, secondary, destructive, and outline options. Each variant provides different visual styling while maintaining consistent typography and accessibility standards across your Dioxus application.

<DemoBadgeVariants />

### Colors

Semantic color variants for status indicators: success (green), warning (orange), and info (blue). Each adapts automatically to light and dark mode using the design system's color tokens.

<DemoBadgeColors />

### Custom

Customize Badge styles with Tailwind CSS classes to match your design system. This example demonstrates how to override default badge styling while preserving component functionality and type safety in Rust UI components.

<DemoBadgeCustom />

## See Also

- [Button](/components/button)
- [Spinner](/components/spinner)
