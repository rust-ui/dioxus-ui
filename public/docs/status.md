---
title: "Status"
name: "status"
cargo_dependencies: []
registry_dependencies: []
type: "components:ui"
path: "ui/status.rs"
description: "Rust/UI component for displaying statuses."
tags: []
---

<DemoStatus />

## Installation

Coming soon.

## Usage

```rust
use crate::ui::status::{Status, StatusVariant};
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
