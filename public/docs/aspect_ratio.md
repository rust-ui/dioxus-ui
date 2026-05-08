+++
title = "Aspect Ratio"
description = "A container that maintains a given aspect ratio for its content."
+++

<DemoAspectRatio />

## Installation

Coming soon.

## Usage

```rust
use crate::ui::aspect_ratio::AspectRatio;
```

```rust
rsx! {
    AspectRatio { ratio: 16.0 / 9.0,
        img { src: "...", alt: "..." }
    }
}
```

## See Also

- [Image](/components/image)
- [Card](/components/card)
