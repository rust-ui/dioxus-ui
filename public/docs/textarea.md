---
title: "Textarea"
name: "textarea"
cargo_dependencies: []
registry_dependencies: []
type: "components:ui"
path: "ui/textarea.rs"
description: "Rust/UI component that displays a textarea."
tags: ["input"]
---

<DemoTextarea />

## Installation

Coming soon.

## Usage

```rust
use crate::ui::textarea::Textarea;
```

```rust
rsx! {
    Textarea { placeholder: "Type your message here..." }
}
```

## Examples

### Disabled

A disabled textarea prevents user input and shows reduced opacity.

<DemoTextareaDisabled />

## See Also

- [Input](/components/input)
- [Label](/components/label)
