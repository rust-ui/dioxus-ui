---
title: Tabs
description: A set of layered sections of content—known as tab panels—displayed one at a time.
tags: ["navigation", "layout"]
---

<DemoTabs />

## Installation

Coming soon.

## Usage

```rust
use crate::ui::tabs::{Tabs, TabsList, TabsTrigger, TabsContent};
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
