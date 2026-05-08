+++
title = "Item"
description = "A flexible list item component with media, content, and action slots."
+++

<DemoItem />

## Installation

Coming soon.

## Usage

```rust
use crate::ui::item::{Item, ItemContent, ItemTitle, ItemDescription, ItemActions, ItemMedia, ItemVariant, ItemSize};
```

```rust
rsx! {
    Item { variant: ItemVariant::Outline,
        ItemContent {
            ItemTitle { "Item Title" }
            ItemDescription { "Item description text." }
        }
        ItemActions {
            Button { "Action" }
        }
    }
}
```

## See Also

- [Card](/components/card)
- [Separator](/components/separator)
- [Badge](/components/badge)
