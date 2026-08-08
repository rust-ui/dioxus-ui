+++
title = "Pressable"
description = "A wrapper that adds press feedback (scale effect) to any children."
+++

<StaticPressable />

## Installation

<StaticInstallPressable />

## Usage

```rust
use crate::ui::pressable::Pressable;
```

```rust
rsx! {
    Pressable {
        Card { "Tap or click me" }
    }
}
```

## See Also

- [Button](/components/button)
- [Card](/components/card)
