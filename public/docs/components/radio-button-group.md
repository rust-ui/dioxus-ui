+++
title = "Radio Button Group"
description = "A CSS-styled radio button group with custom visual selection."
+++

<StaticRadioButtonGroup />

## Installation

<StaticInstallRadioButtonGroup />

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

<StaticRadioButtonGroupRtl />

## See Also

- [Radio Button](/components/radio-button)
- [Checkbox](/components/checkbox)
