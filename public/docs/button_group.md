---
title: "Button Group"
name: "button_group"
cargo_dependencies: []
registry_dependencies: []
type: "components:ui"
path: "ui/button_group.rs"
description: "Rust/UI component that groups buttons together into a cohesive unit."
tags: []
---

# Button Group

## Installation

Coming soon.

## Usage

```rust
use crate::ui::button_group::ButtonGroup;
use crate::ui::button::{Button, ButtonVariant};
```

```rust
rsx! {
    ButtonGroup {
        Button { variant: ButtonVariant::Outline, "First" }
        Button { variant: ButtonVariant::Outline, "Second" }
        Button { variant: ButtonVariant::Outline, "Third" }
    }
}
```

## Examples

### With Separator

<DemoButtonGroupSeparator />

### With Icons

<DemoButtonGroupIcon />

### Sizes

<DemoButtonGroupSizes />

### With Input

<DemoButtonGroupInput />

### RTL

<DemoButtonGroupRtl />

## See Also

- [Button](/components/button)
- [Separator](/components/separator)
