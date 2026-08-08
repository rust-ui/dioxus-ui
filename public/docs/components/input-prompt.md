+++
title = "Input Prompt"
description = "Prompt-style input surface for asking questions and submitting contextual actions in Dioxus."
tags = []
is_new = false
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++

<StaticInputPrompt />

## Installation

<StaticInstallInputPrompt />

## Usage

```rust
use crate::ui::input_prompt::InputPrompt;
```

```rust
rsx! {
    DemoInputPrompt {}
}
```

## Examples

### Basic Prompt

Prompt input surface for short text requests.

<StaticInputPrompt />

### Prompt With Tools

Prompt input combined with auxiliary actions or tools.

<StaticInputPromptWithTools />

## See Also

- [Input Group](/components/input_group)
- [Button](/components/button)
