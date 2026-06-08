+++
title = "Input Phone"
description = "Phone number input with country selection and formatting for Dioxus."
tags = []
is_new = false
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++

<DemoInputPhone />

## Installation

Coming soon.

## Usage

```rust
use crate::ui::input_phone::InputPhone;
```

```rust
rsx! {
    InputPhone {}
}
```

## Examples

### Basic Phone Input

Phone number input with country picker and formatted display.

<DemoInputPhone />

### Disabled

Disabled state for read-only or unavailable phone input workflows.

<DemoInputPhoneDisabled />

## See Also

- [Input](/components/input)
- [Popover](/components/popover)
