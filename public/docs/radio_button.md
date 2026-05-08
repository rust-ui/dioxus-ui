+++
title = "Radio Button"
description = "A signal-driven radio group for selecting a single option from a list."
+++

<DemoRadioButton />

## Installation

Coming soon.

## Usage

```rust
use crate::ui::radio_button::{RadioGroup, RadioGroupItem};
```

```rust
rsx! {
    RadioGroup { value,
        div { class: "flex gap-3 items-center",
            RadioGroupItem { value: "option-a", id: "option-a" }
            Label { html_for: "option-a", "Option A" }
        }
    }
}
```

## See Also

- [Label](/components/label)
- [Checkbox](/components/checkbox)
