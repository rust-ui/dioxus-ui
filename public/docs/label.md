---
title: "Label"
name: "label"
cargo_dependencies: []
registry_dependencies: []
type: "components:ui"
path: "ui/label.rs"
description: "Rust/UI component that displays a label for an input field."
tags: []
---

<DemoLabel />

## Installation

Coming soon.

## Usage

```rust
use crate::ui::label::Label;
```

```rust
rsx! {
    Label { html_for: "email", "Email address" }
}
```

## Examples

### With Input

Pair a label with an input for accessible form fields. The `html_for` prop links the label to the input via its `id`.

## See Also

- [Input](/components/input)
- [Checkbox](/components/checkbox)
