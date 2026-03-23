---
title: Textarea
description: Displays a form textarea or a component that looks like a textarea.
tags: ["form", "input"]
---

<DemoTextarea />

## Installation

Coming soon.

## Usage

```rust
use crate::ui::textarea::Textarea;
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
