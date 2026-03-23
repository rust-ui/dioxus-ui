---
title: Chips
description: Interactive chip component with checkbox state, animated checkmark on selection.
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
