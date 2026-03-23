---
title: Progress
description: Displays an indicator showing the completion progress of a task.
tags: ["feedback", "display"]
---

<DemoProgress />

## Installation

Coming soon.

## Usage

```rust
use crate::ui::progress::Progress;
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
