+++
title = "Marquee"
description = "An infinite scrolling marquee with pause-on-hover support."
+++

<DemoMarquee />

## Installation

<InstallMarquee />

## Usage

```rust
use crate::ui::marquee::{Marquee, MarqueeRow, MarqueeWrapper};
```

```rust
rsx! {
    MarqueeWrapper {
        Marquee {
            MarqueeRow {
                // Items repeat for seamless scroll
                div { "Card 1" }
                div { "Card 2" }
            }
        }
    }
}
```

## See Also

- [Mask](/components/mask)
- [Card](/components/card)
