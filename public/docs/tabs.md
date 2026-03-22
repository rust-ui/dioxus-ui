---
title: Tabs
description: A set of layered sections of content—known as tab panels—displayed one at a time.
tags: ["navigation", "layout"]
---

<DemoTabs />

## Installation

Add `dioxus_ui` to your `Cargo.toml`:

```toml
dioxus_ui = "0.1"
```

## Usage

```rust
use dioxus_ui::tabs::{Tabs, TabsList, TabsTrigger, TabsContent};
```

```rust
rsx! {
    Tabs { default_value: "account",
        TabsList {
            TabsTrigger { value: "account", "Account" }
            TabsTrigger { value: "password", "Password" }
        }
        TabsContent { value: "account",
            p { "Change your account settings here." }
        }
        TabsContent { value: "password",
            p { "Update your password here." }
        }
    }
}
```

## See Also

- [Card](/components/card)
- [Separator](/components/separator)
