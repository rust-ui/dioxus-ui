+++
title = "Radio Button Group"
description = "A CSS-styled radio button group with custom visual selection."
+++

<DemoRadioButtonGroup />

## Installation

Coming soon.

## Usage

```rust
use crate::ui::radio_button_group::{RadioButtonGroup, RadioButton, RadioButtonText};
```

```rust
rsx! {
    RadioButtonGroup {
        RadioButton { checked: true, RadioButtonText { "Option A" } }
        RadioButton { RadioButtonText { "Option B" } }
        RadioButton { RadioButtonText { "Option C" } }
    }
}
```

## Examples

### RTL

<DemoRadioButtonGroupRtl />

## See Also

- [Radio Button](/components/radio-button)
- [Checkbox](/components/checkbox)
