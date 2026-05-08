+++
title = "Bottom Nav"
description = "A mobile bottom navigation bar with touch-optimized buttons."
+++

<DemoBottomNav />

## Installation

Coming soon.

## Usage

```rust
use crate::ui::bottom_nav::{BottomNav, BottomNavGrid, BottomNavButton, BottomNavLabel};
```

```rust
rsx! {
    BottomNav {
        BottomNavGrid {
            BottomNavButton { aria-current: "page",
                House { class: "size-5" }
                BottomNavLabel { "Home" }
            }
        }
    }
}
```

## See Also

- [Tabs](/components/tabs)
- [Button](/components/button)
