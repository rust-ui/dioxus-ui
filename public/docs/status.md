---
title: Status
description: An indicator with a ping animation that shows the live status of an element.
tags: []
---

<DemoStatus />

## Installation

Add `dioxus_ui` to your `Cargo.toml`:

```toml
dioxus_ui = "0.1"
```

## Usage

```rust
use dioxus_ui::status::{Status, StatusVariant};
```

```rust
rsx! {
    Status {
        div { class: "rounded-md size-16 bg-neutral-500" }
    }
}
```

## Variants

<DemoStatusVariants />

Use the `variant` prop to change the indicator color: `Default`, `Active`, `Inactive`, `Normal`.

## See Also

- [Badge](/components/badge)
- [Avatar](/components/avatar)
