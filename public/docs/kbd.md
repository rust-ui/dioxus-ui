---
title: Kbd
description: Displays keyboard shortcut keys with consistent styling.
tags: ["display", "typography"]
---

<DemoKbd />

## Installation

Add `dioxus_ui` to your `Cargo.toml`:

```toml
dioxus_ui = "0.1"
```

## Usage

```rust
use dioxus_ui::kbd::Kbd;
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
