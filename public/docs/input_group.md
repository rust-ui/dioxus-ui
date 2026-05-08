+++
title = "Input Group"
description = "An input with inline prefix/suffix addons such as icons or buttons."
+++

<DemoInputGroup />

## Installation

Coming soon.

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

<DemoInputGroupText />

### Block Layout

<DemoInputGroupBlock />

### RTL

<DemoInputGroupRtl />

## See Also

- [Input](/components/input)
- [Button](/components/button)
