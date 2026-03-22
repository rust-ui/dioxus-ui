---
title: Button
description: Displays a button or a component that looks like a button.
---

## Usage

The `Button` component supports multiple variants, sizes, and states.

## Variants

Use the `variant` prop to change the visual style:

- `default` — filled primary color
- `outline` — bordered, transparent background
- `ghost` — no border, subtle hover
- `destructive` — red, for dangerous actions

## Sizes

| Size | Class     | Height |
|------|-----------|--------|
| `sm` | `btn-sm`  | 32px   |
| `md` | `btn-md`  | 40px   |
| `lg` | `btn-lg`  | 48px   |

## Props

| Prop       | Type              | Default     | Description                        |
|------------|-------------------|-------------|------------------------------------|
| `variant`  | `ButtonVariant`   | `default`   | Visual style of the button         |
| `size`     | `ButtonSize`      | `md`        | Size preset                        |
| `disabled` | `bool`            | `false`     | Disables interaction               |
| `href`     | `Option<String>`  | `None`      | Renders as `<a>` when provided     |
| `class`    | `Option<String>`  | `None`      | Extra Tailwind classes to merge    |

## Ordered steps

1. Import the component
2. Add it to your `rsx!` block
3. Pass props as needed

## Code example

```rust
rsx! {
    Button { variant: ButtonVariant::Outline, "Click me" }
}
```

## Notes

> **Tip:** Buttons render as `<a>` when an `href` prop is provided, keeping full keyboard and screen-reader semantics.

Combine with `tw_merge` to safely override individual Tailwind classes without conflicts.

## Links

See the [Tailwind CSS docs](https://tailwindcss.com) for class reference, and [tw_merge](https://crates.io/crates/tw_merge) for class merging utilities.
