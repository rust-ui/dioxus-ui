---
title: Button
description: Displays a button or a component that looks like a button.
---

## Demo

<DemoButton />

## Variants

<DemoButtonVariants />

## Sizes

<DemoButtonSizes />

## Disabled

<DemoButtonDisabled />

## Stateful

<DemoButtonStateful />

## Reactive

<DemoButtonReactive />

## Class Override

<DemoButtonOverride />

## As Link

<DemoButtonHref />

## Button Group

<DemoButtonGroup />

## Button Group with Icons

<DemoButtonGroupIcon />

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
