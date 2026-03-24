---
title: "Chips"
name: "chips"
cargo_dependencies: []
registry_dependencies: []
type: "components:ui"
path: "ui/chips.rs"
description: "Rust/UI component that displays a chip or a component that looks like a chip."
tags: []
---

<DemoChips />

## Installation

Coming soon.

## Usage

```rust
use crate::ui::chips::{ChipsContainer, ChipItem};
```

```rust
rsx! {
    ChipsContainer {
        ChipItem { label: "sunny" }
        ChipItem { label: "cloudy" }
        ChipItem { label: "hazy" }
    }
}
```

## See Also

- [Badge](/components/badge)
- [Toggle](/components/toggle)
- [Toggle Group](/components/toggle-group)
