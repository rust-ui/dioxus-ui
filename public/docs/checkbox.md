---
title: "Checkbox"
name: "checkbox"
cargo_dependencies: []
registry_dependencies: []
type: "components:ui"
path: "ui/checkbox.rs"
description: "Rust/UI component that displays a control that allows the user to toggle between checked and not checked."
tags: ["utils"]
---

<DemoCheckbox />

## Installation

Coming soon.

## Usage

```rust
use crate::ui::checkbox::Checkbox;
```

```rust
rsx! {
    Checkbox {}
}
```

## Examples

### With Label

Combine the checkbox with a label for accessible form fields.

<DemoCheckboxLabeled />

### Disabled

A disabled checkbox prevents user interaction while maintaining its visual state.

<DemoCheckboxDisabled />

## See Also

- [Switch](/components/switch)
- [Label](/components/label)
