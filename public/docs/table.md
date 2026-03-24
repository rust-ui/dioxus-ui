---
title: "Table"
name: "table"
cargo_dependencies: []
registry_dependencies: []
type: "components:ui"
path: "ui/table.rs"
description: "Rust/UI component that displays a table with header, body and footer."
tags: ["table"]
---

<DemoTable />

## Installation

Coming soon.

## Usage

```rust
use crate::ui::table::{
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
