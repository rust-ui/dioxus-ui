---
title: Skeleton
description: Dioxus UI component that shows a placeholder while content is loading.
tags: []
---

<DemoSkeleton />

## Installation

Add `dioxus_ui` to your `Cargo.toml`:

```toml
dioxus_ui = "0.1"
```

## Usage

```rust
use dioxus_ui::skeleton::Skeleton;
```

```rust
rsx! {
    Skeleton { class: "w-[100px] h-[20px] rounded" }
}
```

## Examples

### Skeleton Image

Skeleton loader placeholder for image content with proper aspect ratio preservation. This example shows how to implement skeleton screens in Dioxus for better perceived performance while card or avatar content loads.

<DemoSkeletonImage />

### Skeleton Avatar

Placeholder pattern for lists of user profiles — circle avatar combined with name and subtitle lines.

<DemoSkeletonAvatar />

### Skeleton Text

Multi-line text block placeholder with varying widths to simulate natural paragraph flow.

<DemoSkeletonText />

### Skeleton Form

Placeholder for form layouts showing label and input field pairs while content loads.

<DemoSkeletonForm />

### Skeleton Table

Row-based placeholder for tabular data with varying cell widths to simulate realistic table content.

<DemoSkeletonTable />

## See Also

- [Spinner](/components/spinner)
- [Card](/components/card)
