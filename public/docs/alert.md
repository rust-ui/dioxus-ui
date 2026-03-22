---
title: Alert
description: Displays a callout for user attention with support for default and destructive variants.
tags: ["feedback", "display"]
---

<DemoAlert />

## Installation

Add `dioxus_ui` to your `Cargo.toml`:

```toml
dioxus_ui = "0.1"
```

## Usage

```rust
use dioxus_ui::alert::{Alert, AlertTitle, AlertDescription};
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

<DemoAlertDestructive />

## See Also

- [Badge](/components/badge)
- [Spinner](/components/spinner)
