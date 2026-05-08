+++
title = "Animate"
description = "A wrapper that applies entrance animations and hover animations via CSS."
+++

<DemoAnimate />

## Installation

Coming soon.

## Usage

```rust
use crate::ui::animate::{Animate, AnimateHoverVariant, AnimateVariant};
```

```rust
rsx! {
    Animate { hover_variant: AnimateHoverVariant::Wobble,
        span { "Hover me" }
    }
}
```

## Examples

### Staggered Group

```rust
use crate::ui::animate::{AnimateGroup, AnimateGroupItem, AnimateVariant};

rsx! {
    AnimateGroup {
        AnimateGroupItem { variant: AnimateVariant::FadeUp, delay_ms: 0, "First" }
        AnimateGroupItem { variant: AnimateVariant::FadeUp, delay_ms: 100, "Second" }
        AnimateGroupItem { variant: AnimateVariant::FadeUp, delay_ms: 200, "Third" }
    }
}
```

## See Also

- [Badge](/components/badge)
- [Card](/components/card)
