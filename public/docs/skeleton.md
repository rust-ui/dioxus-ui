---
title: Skeleton
description: Use to show a placeholder while content is loading.
---

## Demo

<DemoSkeleton />

## Text

<DemoSkeletonText />

## Avatar

<DemoSkeletonAvatar />

## Form

<DemoSkeletonForm />

## Image

<DemoSkeletonImage />

## Table

<DemoSkeletonTable />

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
