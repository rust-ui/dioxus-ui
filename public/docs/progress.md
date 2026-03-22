---
title: Progress
description: Displays an indicator showing the completion progress of a task.
tags: ["feedback", "display"]
---

<DemoProgress />

## Installation

Add `dioxus_ui` to your `Cargo.toml`:

```toml
dioxus_ui = "0.1"
```

## Usage

```rust
use dioxus_ui::progress::Progress;
```

```rust
rsx! {
    Progress { value: 60.0 }
}
```

## Examples

### Animated

Use a Dioxus signal to drive the progress value dynamically.

<DemoProgressAnimated />

## See Also

- [Spinner](/components/spinner)
- [Skeleton](/components/skeleton)
