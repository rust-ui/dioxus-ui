---
title: Textarea
description: Displays a form textarea or a component that looks like a textarea.
tags: ["form", "input"]
---

<DemoTextarea />

## Installation

Add `dioxus_ui` to your `Cargo.toml`:

```toml
dioxus_ui = "0.1"
```

## Usage

```rust
use dioxus_ui::textarea::Textarea;
```

```rust
rsx! {
    Textarea { placeholder: "Type your message here..." }
}
```

## Examples

### Disabled

A disabled textarea prevents user input and shows reduced opacity.

<DemoTextareaDisabled />

## See Also

- [Input](/components/input)
- [Label](/components/label)
