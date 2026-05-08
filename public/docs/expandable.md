+++
title = "Expandable"
description = "A trigger that expands to reveal additional content with a close button."
+++

<DemoExpandable />

## Installation

Coming soon.

## Usage

```rust
use crate::ui::expandable::{ExpandableTrigger, ExpandableTransition, ExpandableContent};
```

```rust
rsx! {
    ExpandableTrigger {
        ExpandableTransition {
            span { "Open" }
        }
        ExpandableContent {
            p { "Expanded content here." }
        }
    }
}
```

## See Also

- [Collapsible](/components/collapsible)
- [Dialog](/components/dialog)
