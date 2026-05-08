+++
title = "Bento Grid"
description = "A CSS grid layout for bento-style card arrangements."
+++

<DemoBentoGrid />

## Installation

Coming soon.

## Usage

```rust
use crate::ui::bento_grid::{BentoGrid, BentoGrid6, BentoRow, BentoCell};
```

```rust
rsx! {
    BentoGrid {
        BentoRow { class: "md:col-span-2",
            BentoCell { "Large cell" }
        }
        BentoRow {
            BentoCell { "Small cell" }
        }
    }
}
```

## Examples

### 6-Column

<DemoBentoGrid6 />

## See Also

- [Card](/components/card)
- [Separator](/components/separator)
