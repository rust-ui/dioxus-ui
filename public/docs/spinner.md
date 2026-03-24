---
title: "Spinner"
name: "spinner"
cargo_dependencies: []
registry_dependencies: []
type: "components:ui"
path: "ui/spinner.rs"
description: "A loading spinner component with animation for indicating processing states."
tags: ["animation", "utils"]
---

<DemoSpinner />

## Installation

Coming soon.

## Usage

```rust
use crate::ui::spinner::Spinner;
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
