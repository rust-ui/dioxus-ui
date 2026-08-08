+++
title = "Field"
description = "Form field layout components for labels, descriptions, and error messages."
+++

<StaticField />

## Installation

<StaticInstallField />

## Usage

```rust
use crate::ui::field::{Field, FieldGroup, FieldLabel, FieldDescription, FieldError};
```

```rust
rsx! {
    FieldGroup {
        Field {
            FieldLabel { html_for: "email", "Email" }
            Input { id: "email", placeholder: "you@example.com" }
            FieldDescription { "We'll never share your email." }
        }
    }
}
```

## Examples

### RTL

<StaticFieldRtl />

## See Also

- [Input](/components/input)
- [Label](/components/label)
- [Checkbox](/components/checkbox)
