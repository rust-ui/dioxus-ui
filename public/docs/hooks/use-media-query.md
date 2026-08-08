+++
title = "Use Media Query"
description = "A reactive hook that tracks whether a CSS media query matches, updating automatically when the viewport changes."
tags = ["utils"]
is_new = false
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++

<StaticUseMediaQuery />

## Installation

<StaticInstallUseMediaQuery />

## Usage

```rust
use registry::hooks::use_media_query::use_media_query;
```

```rust
let is_wide = use_media_query("(min-width: 1024px)");

rsx! {
    if is_wide() { "Wide layout" } else { "Narrow layout" }
}
```

## Examples

### Default

<StaticUseMediaQuery />

## See Also

- [Use Is Mobile](/hooks/use-is-mobile)
