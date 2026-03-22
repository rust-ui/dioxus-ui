---
title: Spinner
description: A loading spinner component with animation for indicating processing states.
tags: ["animation"]
---

<DemoSpinner />

## Installation

Add `dioxus_ui` to your `Cargo.toml`:

```toml
dioxus_ui = "0.1"
```

## Usage

```rust
use dioxus_ui::spinner::Spinner;
```

```rust
rsx! {
    Spinner {}
}
```

## Examples

### In Button

Loading spinner integrated within button components for async action feedback. This example demonstrates how to combine Spinner and [Button](/components/button) in Dioxus to create accessible loading states with proper visual indicators.

<DemoSpinnerButton />

## See Also

- [Skeleton](/components/skeleton)
- [Button](/components/button)
