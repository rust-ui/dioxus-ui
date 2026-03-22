---
title: Input
description: Dioxus UI component that displays an input field that allows the user to enter text.
tags: ["input"]
---

<DemoInput />

## Usage

You can use the `Input` component in combination with the [Button](/components/button) component.

```rust
use dioxus_ui::input::Input;
```

```rust
rsx! {
    Input { placeholder: "Enter text..." }
}
```

## Examples

### Input Copy

Input field with integrated copy-to-clipboard functionality for easy text sharing. This example shows how to build interactive input components in Dioxus with [Button](/components/button) integration and clipboard API.

<DemoInputCopy />

## Installation

Add `dioxus_ui` to your `Cargo.toml`:

```toml
dioxus_ui = "0.1"
```

## See Also

- [Button](/components/button)
- [Card](/components/card)
