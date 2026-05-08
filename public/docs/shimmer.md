+++
title = "Shimmer"
description = "A loading shimmer effect overlay driven by a signal."
+++

<DemoShimmer />

## Installation

Coming soon.

## Usage

```rust
use crate::ui::shimmer::Shimmer;
```

```rust
rsx! {
    Shimmer { loading: ReadOnlySignal::new(loading),
        Card { "Content that shimmers while loading" }
    }
}
```

## See Also

- [Skeleton](/components/skeleton)
- [Spinner](/components/spinner)
