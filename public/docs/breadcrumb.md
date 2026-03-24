+++
title = "Breadcrumb"
description = "Rust/UI component that displays the path to the current resource using a hierarchy of links."
+++

<DemoBreadcrumb />

## Installation

Coming soon.

## Usage

```rust
use crate::ui::breadcrumb::{
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


### RTL

<DemoBreadcrumbRtl />

## See Also

- [Tabs](/components/tabs)
- [Button](/components/button)
