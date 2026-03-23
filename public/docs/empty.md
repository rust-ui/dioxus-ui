---
title: Empty
description: An empty state component for displaying a placeholder when there is no content.
tags: []
---

<DemoEmpty />

## Installation

Coming soon.

## Usage

```rust
use crate::ui::empty::{Empty, EmptyHeader, EmptyTitle, EmptyDescription, EmptyContent, EmptyMedia, EmptyMediaVariant};
```

```rust
rsx! {
    Empty {
        EmptyHeader {
            EmptyMedia { variant: EmptyMediaVariant::Icon,
                // your icon here
            }
            EmptyTitle { "No Projects Yet" }
            EmptyDescription { "Get started by creating your first project." }
        }
        EmptyContent {
            Button { "Create Project" }
        }
    }
}
```

## Muted

<DemoEmptyMuted />

## See Also

- [Card](/components/card)
- [Skeleton](/components/skeleton)
