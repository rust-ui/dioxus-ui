+++
title = "Scroll Area"
description = "A scrollable container with a custom scrollbar."
+++

<DemoScrollArea />

## Installation

Coming soon.

## Usage

```rust
use crate::ui::scroll_area::ScrollArea;
```

```rust
rsx! {
    ScrollArea { class: "h-72",
        div { "Content..." }
    }
}
```

## Examples

### Horizontal

<DemoScrollAreaHorizontal />

### RTL

<DemoScrollAreaRtl />

## See Also

- [Separator](/components/separator)
- [Card](/components/card)
