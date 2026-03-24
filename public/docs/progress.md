+++
title = "Progress"
description = "Rust/UI component that displays a progress bar indicating task completion."
+++

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

## See Also

- [Spinner](/components/spinner)
- [Skeleton](/components/skeleton)
