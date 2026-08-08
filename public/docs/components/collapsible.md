+++
title = "Collapsible"
description = "An interactive component which expands/collapses a panel with smooth animation."
+++

<StaticCollapsible />

## Installation

<StaticInstallCollapsible />

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


### Settings

<StaticCollapsibleSettings />

## See Also

- [Accordion](/components/accordion)
- [Tabs](/components/tabs)
