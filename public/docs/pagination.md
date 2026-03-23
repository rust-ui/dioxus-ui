---
title: Pagination
description: Navigation component for moving between pages of content.
tags: []
---

<DemoPagination />

## Installation

Add `dioxus_ui` to your `Cargo.toml`:

```toml
dioxus_ui = "0.1"
```

## Usage

```rust
use dioxus_ui::pagination::{
    Pagination, PaginationList, PaginationItem,
    PaginationLink, PaginationPrev, PaginationNext,
};
```

```rust
let mut page = use_signal(|| 1u32);

rsx! {
    Pagination {
        PaginationList {
            PaginationItem {
                PaginationPrev {
                    disabled: page() == 1,
                    onclick: move |_| { if page() > 1 { page.set(page() - 1); } },
                }
            }
            for p in 1..=5 {
                PaginationItem {
                    PaginationLink {
                        page: p,
                        is_active: page() == p,
                        onclick: move |_| page.set(p),
                    }
                }
            }
            PaginationItem {
                PaginationNext {
                    disabled: page() == 5,
                    onclick: move |_| { if page() < 5 { page.set(page() + 1); } },
                }
            }
        }
    }
}
```

## See Also

- [Button](/components/button)
- [Tabs](/components/tabs)
