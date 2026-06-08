+++
title = "Form"
description = "Typed form composition primitives for Dioxus with field context and validation helpers."
tags = []
is_new = false
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++

<DemoForm />

## Installation

Coming soon.

## Usage

```rust
use crate::ui::form::{
    Form,
    FormContent,
    FormDescription,
    FormError,
    FormField,
    FormGroup,
    FormInput,
    FormLabel,
    FormProvider,
    FormSet,
};
use crate::hooks::use_form::use_form;
```

```rust
rsx! {
    DemoForm {}
}
```

## Examples

### Basic Form

Typed form composition with labels, inputs, and grouped sections.

<DemoForm />

### Validation

Validation-oriented example showing errors and touched-field behavior.

<DemoFormValidation />

### Group

Grouped form layout for related inputs.

<DemoFormGroup />

### Fieldset

Fieldset-style form grouping with legends and descriptions.

<DemoFormFieldset />

### Error

Inline form error presentation.

<DemoFormError />

### Select

Form usage with select controls.

<DemoFormSelect />

### Auto Form

Auto-generated form fields from a typed schema.

<DemoAutoForm />

## See Also

- [Field](/components/field)
- [Input](/components/input)
