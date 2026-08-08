+++
title = "Multi Select"
description = "Multi-value selection component for Dioxus."
tags = []
is_new = false
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++

<StaticMultiSelect />

## Installation

<StaticInstallMultiSelect />

## Usage

```rust
use crate::ui::multi_select::{
    MultiSelect,
    MultiSelectContent,
    MultiSelectGroup,
    MultiSelectItem,
    MultiSelectOption,
    MultiSelectTrigger,
    MultiSelectValue,
};
```

```rust
rsx! {
    DemoMultiSelect {}
}
```

## Examples

### Basic Multi Select

Select multiple values from a compact popover list.

<StaticMultiSelect />

### Alignment

Start and end aligned menu positioning relative to the trigger.

<StaticMultiSelectAlign />

### Scrollable

Scrollable option list for larger datasets like timezones.

<StaticMultiSelectScrollable />

## See Also

- [Select](/components/select)
- [Combobox](/components/combobox)
