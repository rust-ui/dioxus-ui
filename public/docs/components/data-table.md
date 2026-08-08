+++
title = "Data Table"
description = "Structured data table demos for sorting, filtering, and tabular presentation in Dioxus."
tags = []
is_new = false
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++

<StaticDataTable />

## Installation

<StaticInstallDataTable />

## Usage

```rust
use crate::ui::table::Table;
```

```rust
rsx! {
    DemoDataTable {}
}
```

## Examples

### Basic Data Table

Simple structured data table for displaying rows and columns with consistent styling.

<StaticDataTable />

### Filtered Table

Data table with interactive filtering controls for narrowing large datasets.

<StaticDataTableFilters />

## See Also

- [Table](/components/table)
- [Pagination](/components/pagination)
