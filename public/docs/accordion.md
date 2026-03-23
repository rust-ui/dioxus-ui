---
title: Accordion
description: A vertically stacked set of interactive headings that each reveal a section of content.
tags: []
---

<DemoAccordion />

## Installation

Add `dioxus_ui` to your `Cargo.toml`:

```toml
dioxus_ui = "0.1"
```

## Usage

```rust
use dioxus_ui::accordion::{Accordion, AccordionItem, AccordionTrigger, AccordionContent};
```

```rust
let mut open = use_signal(|| false);

rsx! {
    Accordion {
        AccordionItem {
            AccordionTrigger {
                open: open.read_only(),
                onclick: move |_| open.toggle(),
                "Is it accessible?"
            }
            AccordionContent {
                open: open.read_only(),
                "Yes. It adheres to the WAI-ARIA design pattern."
            }
        }
    }
}
```

## See Also

- [Collapsible](/components/collapsible)
- [Tabs](/components/tabs)
