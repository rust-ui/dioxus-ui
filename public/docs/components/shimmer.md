+++
title = "Shimmer"
description = "A loading shimmer effect overlay driven by a signal."
+++

<StaticShimmer />

## Installation

<StaticInstallShimmer />

## Usage

```rust
use crate::ui::shimmer::Shimmer;
```

```rust
rsx! {
    Shimmer { loading: ReadSignal::new(loading),
        Card { "Content that shimmers while loading" }
    }
}
```

## See Also

- [Skeleton](/components/skeleton)
- [Spinner](/components/spinner)
