---
title: Table
description: A responsive table component for displaying structured data.
tags: []
---

<DemoTable />

## Installation

Add `dioxus_ui` to your `Cargo.toml`:

```toml
dioxus_ui = "0.1"
```

## Usage

```rust
use dioxus_ui::table::{
    Table, TableHeader, TableBody, TableRow,
    TableHead, TableCell, TableCaption,
};
```

```rust
rsx! {
    Table {
        TableHeader {
            TableRow {
                TableHead { "Name" }
                TableHead { "Email" }
            }
        }
        TableBody {
            TableRow {
                TableCell { "Alice" }
                TableCell { "alice@example.com" }
            }
        }
    }
}
```

## See Also

- [Card](/components/card)
- [Badge](/components/badge)
