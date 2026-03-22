---
title: Button
description: Displays a button or a component that looks like a button.
---

## Demo

<demo-button></demo-button>

## Variants

<demo-button-variants></demo-button-variants>

## Sizes

<demo-button-sizes></demo-button-sizes>

## Disabled

<demo-button-disabled></demo-button-disabled>

## Stateful

<demo-button-stateful></demo-button-stateful>

## Reactive

<demo-button-reactive></demo-button-reactive>

## Class Override

<demo-button-override></demo-button-override>

## As Link

<demo-button-href></demo-button-href>

## Button Group

<demo-button-group></demo-button-group>

## Button Group with Icons

<demo-button-group-icon></demo-button-group-icon>

## Props

| Prop       | Type             | Default   | Description                     |
|------------|------------------|-----------|---------------------------------|
| `variant`  | `ButtonVariant`  | `default` | Visual style                    |
| `size`     | `ButtonSize`     | `md`      | Size preset                     |
| `disabled` | `bool`           | `false`   | Disables interaction            |
| `href`     | `Option<String>` | `None`    | Renders as `<a>` when provided  |
| `class`    | `Option<String>` | `None`    | Extra Tailwind classes to merge |

## Usage

```rust
rsx! {
    Button { variant: ButtonVariant::Outline, "Click me" }
}
```

> **Tip:** Combine with `tw_merge` to safely override individual Tailwind classes.
