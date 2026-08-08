+++
title = "Use Is Mobile"
description = "A reactive hook that returns true when the viewport is below the mobile breakpoint (768px)."
tags = ["utils"]
is_new = false
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++

<StaticUseIsMobile />

## Installation

<StaticInstallUseIsMobile />

## Usage

```rust
use registry::hooks::use_is_mobile::use_is_mobile;
```

```rust
let is_mobile = use_is_mobile();

rsx! {
    if is_mobile() {
        div { "Mobile layout" }
    } else {
        div { "Desktop layout" }
    }
}
```

## Examples

### Default

<StaticUseIsMobile />

## See Also

- [Use Media Query](/hooks/use-media-query)
- [Drawer](/components/drawer)
- [Dialog](/components/dialog)
