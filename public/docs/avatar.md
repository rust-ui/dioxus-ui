---
title: Avatar
description: An image element with a fallback for representing the user.
tags: ["display", "user"]
---

<DemoAvatar />

## Installation

Add `dioxus_ui` to your `Cargo.toml`:

```toml
dioxus_ui = "0.1"
```

## Usage

```rust
use dioxus_ui::avatar::{Avatar, AvatarImage, AvatarFallback};
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
