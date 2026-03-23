---
title: Collapsible
description: An interactive component which expands/collapses a panel.
tags: []
---

<DemoCollapsible />

## Installation

Add `dioxus_ui` to your `Cargo.toml`:

```toml
dioxus_ui = "0.1"
```

## Usage

```rust
use dioxus_ui::collapsible::{Collapsible, CollapsibleTrigger, CollapsibleContent};
```

```rust
let mut open = use_signal(|| false);

rsx! {
    Collapsible {
        CollapsibleTrigger {
            open: open.read_only(),
            onclick: move |_| open.toggle(),
            "Toggle"
        }
        CollapsibleContent {
            open: open.read_only(),
            "Hidden content revealed on open."
        }
    }
}
```

## See Also

- [Accordion](/components/accordion)
- [Tabs](/components/tabs)
