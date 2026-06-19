---
title: "Data Table"
name: "data_table"
cargo_dependencies: []
registry_dependencies: []
type: "components:ui"
path: "ui/data_table.rs"
description: "Structured data table demos for sorting, filtering, and tabular presentation in Dioxus."
tags: []
---

# Data Table

Structured data table demos for sorting, filtering, and tabular presentation in Dioxus.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add data_table
```

## Component Code

```rust
// * Reuse @table.rs
pub use crate::components::ui::table::{
    Table as DataTable, TableBody as DataTableBody, TableCaption as DataTableCaption, TableCell as DataTableCell,
    TableFooter as DataTableFooter, TableHead as DataTableHead, TableHeader as DataTableHeader,
    TableRow as DataTableRow, TableWrapper as DataTableWrapper,
};
```
