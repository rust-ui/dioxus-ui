---
title: Label
description: Renders an accessible label associated with controls.
tags: ["form", "accessibility"]
---

<DemoLabel />

## Installation

Coming soon.

## Usage

```rust
use crate::ui::label::Label;
```

```rust
rsx! {
    Label { html_for: "email", "Email address" }
}
```

## Examples

### With Input

Pair a label with an input for accessible form fields. The `html_for` prop links the label to the input via its `id`.

<DemoLabelInput />

## See Also

- [Input](/components/input)
- [Checkbox](/components/checkbox)
