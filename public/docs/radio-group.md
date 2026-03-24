+++
title = "Radio Group"
description = "Rust/UI component that displays a set of radio buttons where only one can be selected at a time."
+++

<DemoRadioGroup />

## Installation

Coming soon.

## Usage

```rust
use crate::ui::radio_group::{RadioGroup, RadioItem};
```

```rust
rsx! {
    RadioGroup {
        div { class: "flex gap-3 items-center",
            RadioItem { name: "plan", value: "free", checked: true, onchange: |_| {} }
            Label { "Free" }
        }
        div { class: "flex gap-3 items-center",
            RadioItem { name: "plan", value: "pro", checked: false, onchange: |_| {} }
            Label { "Pro" }
        }
    }
}
```

## Examples

### Custom Style

<DemoRadioButtonCustom />

## See Also

- [Checkbox](/components/checkbox)
- [Label](/components/label)
