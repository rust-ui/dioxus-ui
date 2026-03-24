---
title: "Alert"
name: "alert"
cargo_dependencies: []
registry_dependencies: []
type: "components:ui"
path: "ui/alert.rs"
description: "Rust/UI component that displays a callout to the user."
tags: []
---

<DemoAlert />

## Installation

Coming soon.

## Usage

```rust
use crate::ui::alert::{Alert, AlertTitle, AlertDescription};
```

```rust
rsx! {
    Alert {
        AlertTitle { "Heads up!" }
        AlertDescription { "You can add components to your app using the CLI." }
    }
}
```

## Examples

### Destructive

Use the destructive variant to communicate errors or critical warnings to the user.

## See Also

- [Badge](/components/badge)
- [Spinner](/components/spinner)
