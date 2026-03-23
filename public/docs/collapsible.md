---
title: Collapsible
description: An interactive component which expands/collapses a panel.
tags: []
---

<DemoCollapsible />

## Installation

Coming soon.

## Usage

```rust
use crate::ui::collapsible::{Collapsible, CollapsibleTrigger, CollapsibleContent};
```

```rust
let mut open = use_signal(|| false);

rsx! {
    Collapsible {
        CollapsibleTrigger {
            open: open,
            onclick: move |_| open.toggle(),
            "Toggle"
        }
        CollapsibleContent {
            open: open,
            "Hidden content revealed on open."
        }
    }
}
```

## See Also

- [Accordion](/components/accordion)
- [Tabs](/components/tabs)
