+++
title = "Avatar"
description = "Rust/UI component that displays an avatar with image and fallback support."
+++

<DemoAvatar />

## Installation

Coming soon.

## Usage

```rust
use crate::ui::avatar::{Avatar, AvatarImage, AvatarFallback};
```

```rust
rsx! {
    Avatar {
        AvatarImage { src: "https://github.com/shadcn.png", alt: "@shadcn" }
        AvatarFallback { "CN" }
    }
}
```

## Examples

### Fallback

When the image fails to load or no `src` is provided, the fallback content is displayed instead.

### Sizes

Avatar supports three size variants: `Sm`, `Default`, and `Lg`.


### Group Count with Icon

<DemoAvatarGroupCountIcon />

## See Also

- [Badge](/components/badge)
- [Skeleton](/components/skeleton)
