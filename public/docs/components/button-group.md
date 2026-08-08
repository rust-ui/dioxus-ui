+++
title = "Button Group"
description = "Rust/UI component that groups buttons together into a cohesive unit."
+++

# Button Group

## Installation

<StaticInstallButtonGroup />

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

<StaticButtonGroupSeparator />

### With Icons

<StaticButtonGroupIcon />

### Sizes

<StaticButtonGroupSizes />

### With Input

<StaticButtonGroupInput />

### RTL

<StaticButtonGroupRtl />

## See Also

- [Button](/components/button)
- [Separator](/components/separator)
