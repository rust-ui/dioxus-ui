---
title: Skeleton
description: Use to show a placeholder while content is loading.
---

## Demo

<demo-skeleton></demo-skeleton>

## Text

<demo-skeleton-text></demo-skeleton-text>

## Avatar

<demo-skeleton-avatar></demo-skeleton-avatar>

## Form

<demo-skeleton-form></demo-skeleton-form>

## Image

<demo-skeleton-image></demo-skeleton-image>

## Table

<demo-skeleton-table></demo-skeleton-table>

## Props

| Prop    | Type             | Default | Description             |
|---------|------------------|---------|-------------------------|
| `class` | `Option<String>` | `None`  | Extra Tailwind classes  |

## Usage

```rust
rsx! {
    Skeleton { class: "h-4 w-48 rounded" }
}
```
