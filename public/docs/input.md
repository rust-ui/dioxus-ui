---
title: Input
description: Displays a form input field for user text entry.
---

## Demo

<demo-input></demo-input>

## With Copy

<demo-input-copy></demo-input-copy>

## Props

| Prop          | Type              | Default  | Description                    |
|---------------|-------------------|----------|--------------------------------|
| `placeholder` | `Option<String>`  | `None`   | Placeholder text               |
| `value`       | `Option<String>`  | `None`   | Controlled value               |
| `input_type`  | `InputType`       | `text`   | Input type (text, password...) |
| `disabled`    | `bool`            | `false`  | Disables the input             |
| `class`       | `Option<String>`  | `None`   | Extra Tailwind classes         |

## Usage

```rust
rsx! {
    Input { placeholder: "Email address" }
}
```
