+++
title = "Input Group"
description = "An input with inline prefix/suffix addons such as icons or buttons."
+++

<StaticInputGroup />

## Installation

<StaticInstallInputGroup />

## Usage

```rust
use crate::ui::input_group::{InputGroup, InputGroupAddon, InputGroupInput};
```

```rust
rsx! {
    InputGroup {
        InputGroupInput { placeholder: "Search..." }
        InputGroupAddon {
            Search { class: "size-4" }
        }
    }
}
```

## Examples

### Text Addons

<StaticInputGroupText />

### Block Layout

<StaticInputGroupBlock />

### RTL

<StaticInputGroupRtl />

## See Also

- [Input](/components/input)
- [Button](/components/button)
