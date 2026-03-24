+++
title = "Chips"
description = "Rust/UI component that displays a chip or a component that looks like a chip."
+++

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
