---
title: Switch
description: A control that allows the user to toggle between checked and unchecked states.
tags: ["interactive", "form"]
---

<DemoSwitch />

## Installation

Add `dioxus_ui` to your `Cargo.toml`:

```toml
dioxus_ui = "0.1"
```

## Usage

```rust
use dioxus_ui::switch::Switch;
```

```rust
rsx! {
    Switch {}
}
```

## Examples

### With Label

Pair a switch with a label for a clear, accessible toggle control.

<DemoSwitchLabeled />

## See Also

- [Checkbox](/components/checkbox)
- [Label](/components/label)
