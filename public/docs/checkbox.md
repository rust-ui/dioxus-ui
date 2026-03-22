---
title: Checkbox
description: A control that allows the user to toggle between checked and unchecked states.
tags: ["interactive", "form"]
---

<DemoCheckbox />

## Installation

Add `dioxus_ui` to your `Cargo.toml`:

```toml
dioxus_ui = "0.1"
```

## Usage

```rust
use dioxus_ui::checkbox::Checkbox;
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
