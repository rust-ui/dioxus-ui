---
title: Kbd
description: Displays keyboard shortcut keys with consistent styling.
tags: ["display", "typography"]
---

<DemoKbd />

## Installation

Coming soon.

## Usage

```rust
use crate::ui::kbd::Kbd;
```

```rust
rsx! {
    Kbd { "⌘K" }
}
```

## Examples

### Key Combination

Combine multiple `Kbd` elements with a separator to represent multi-key shortcuts.

<DemoKbdCombination />

## See Also

- [Badge](/components/badge)
- [Button](/components/button)
