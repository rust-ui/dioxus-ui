---
title: Breadcrumb
description: Displays the path to the current resource using a hierarchy of links.
tags: []
---

<DemoBreadcrumb />

## Installation

Add `dioxus_ui` to your `Cargo.toml`:

```toml
dioxus_ui = "0.1"
```

## Usage

```rust
use dioxus_ui::breadcrumb::{
    Breadcrumb, BreadcrumbList, BreadcrumbItem,
    BreadcrumbLink, BreadcrumbPage, BreadcrumbSeparator,
};
```

```rust
rsx! {
    Breadcrumb {
        BreadcrumbList {
            BreadcrumbItem {
                BreadcrumbLink { href: "/", "Home" }
            }
            BreadcrumbSeparator {}
            BreadcrumbItem {
                BreadcrumbPage { "Components" }
            }
        }
    }
}
```

## See Also

- [Tabs](/components/tabs)
- [Button](/components/button)
