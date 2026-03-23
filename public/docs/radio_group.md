---
title: Radio Group
description: A set of checkable buttons—known as radio buttons—where no more than one of the buttons can be checked at a time.
tags: []
---

<DemoRadioGroup />

## Installation

Add `dioxus_ui` to your `Cargo.toml`:

```toml
dioxus_ui = "0.1"
```

## Usage

```rust
use dioxus_ui::radio_group::{RadioGroup, RadioItem};
```

```rust
let mut selected = use_signal(|| "default".to_string());

rsx! {
    RadioGroup {
        RadioItem {
            name: "spacing",
            value: "default",
            checked: selected() == "default",
            onchange: move |_| selected.set("default".to_string()),
            "Default"
        }
        RadioItem {
            name: "spacing",
            value: "compact",
            checked: selected() == "compact",
            onchange: move |_| selected.set("compact".to_string()),
            "Compact"
        }
    }
}
```

## See Also

- [Checkbox](/components/checkbox)
- [Switch](/components/switch)
