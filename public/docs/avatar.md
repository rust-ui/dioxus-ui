---
title: Avatar
description: An image element with a fallback for representing the user.
tags: ["display", "user"]
---

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

<DemoAvatarFallback />

### Sizes

Avatar supports three size variants: `Sm`, `Default`, and `Lg`.

<DemoAvatarSizes />

## See Also

- [Badge](/components/badge)
- [Skeleton](/components/skeleton)
