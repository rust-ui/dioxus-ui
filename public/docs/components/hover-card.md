+++
title = "Hover Card"
description = "A floating card that appears on hover using CSS anchor positioning."
+++

<StaticHoverCard />

## Installation

<StaticInstallHoverCard />

## Usage

```rust
use crate::ui::hover_card::{HoverCard, HoverCardTrigger, HoverCardContent};
```

```rust
rsx! {
    HoverCard {
        HoverCardTrigger {
            Button { variant: ButtonVariant::Link, "@username" }
        }
        HoverCardContent {
            p { "Card content" }
        }
    }
}
```

## Examples

### RTL

<StaticHoverCardRtl />

## See Also

- [Button](/components/button)
- [Avatar](/components/avatar)
