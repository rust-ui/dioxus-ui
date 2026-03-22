---
title: Button
description: Dioxus UI component that displays a button or a component that looks like a button.
tags: ["button"]
---

<DemoButton />

## Installation

Add `dioxus_ui` to your `Cargo.toml`:

```toml
dioxus_ui = "0.1"
```

## Usage

```rust
use dioxus_ui::button::Button;
```

```rust
rsx! {
    Button { "Click me" }
}
```

## Examples

### Variants

Available Button style variants include default, secondary, outline, ghost, destructive, and link. Each variant provides different visual styling while maintaining consistent behavior and accessibility across your Dioxus application.

<DemoButtonVariants />

### Sizes

Button size options include small, default, and large. This example shows how to implement responsive button sizing in your Rust UI components to match different design requirements and use cases.

<DemoButtonSizes />

### Disabled

Disabled Button state with proper ARIA attributes for accessibility. This example demonstrates the correct way to implement non-interactive button states while maintaining semantic HTML and screen reader compatibility.

<DemoButtonDisabled />

### Stateful

Button component that manages its own internal state using Dioxus signals. This example demonstrates building self-contained interactive components with local state management, perfect for toggles and loading states.

<DemoButtonStateful />

### Reactive

Button component that updates dynamically using Dioxus signals to respond to state changes. This example demonstrates how to build reactive UI components in Rust that automatically re-render when underlying data changes.

<DemoButtonReactive />

### Overriding Button

Customize Button styles by overriding default properties with custom classes. This example shows how to extend the base button component while preserving type safety and component composition patterns in Dioxus.

<DemoButtonOverride />

### With Href

Automatic conversion to semantic `<a>` tag when using the `href` prop. The button component intelligently switches between button and anchor elements for proper HTML semantics and SEO.

<DemoButtonHref />

### Button Group

Group multiple buttons together with consistent spacing and shared borders. Useful for toolbars, segmented controls, and pagination.

<DemoButtonGroup />

### Button Group with Icons

Button group variant combining text labels with Lucide icons for enhanced visual clarity and compact action sets.

<DemoButtonGroupIcon />

## See Also

- [Input](/components/input)
- [Badge](/components/badge)
