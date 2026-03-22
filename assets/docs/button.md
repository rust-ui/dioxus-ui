---
title: Button
description: Displays a button or a component that looks like a button.
---

## Demo

<demo-button></demo-button>

## Variants

<demo-button-variants></demo-button-variants>

Use the `variant` prop to change the visual style:

| Variant        | Description                    |
|----------------|--------------------------------|
| `default`      | Filled primary color           |
| `secondary`    | Muted secondary style          |
| `outline`      | Bordered, transparent bg       |
| `ghost`        | No border, subtle hover        |
| `destructive`  | Red, for dangerous actions     |
| `link`         | Looks like a hyperlink         |

## Sizes

| Size | Height |
|------|--------|
| `sm` | 32px   |
| `md` | 40px   |
| `lg` | 48px   |

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
