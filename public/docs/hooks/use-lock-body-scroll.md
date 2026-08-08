+++
title = "Use Lock Body Scroll"
description = "A signal-based hook that locks and unlocks body scrolling, useful for dialogs, sheets, and overlays."
tags = ["utils", "dialog"]
is_new = false
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++

<StaticUseLockBodyScroll />

## Installation

<StaticInstallUseLockBodyScroll />

## Usage

```rust
use registry::hooks::use_lock_body_scroll::use_lock_body_scroll;
```

```rust
let mut scroll_locked = use_lock_body_scroll(false);

scroll_locked.set(true);
scroll_locked.set(false);
```

## Return Value

- `Signal<bool>`: `true` when body scrolling is locked, `false` when unlocked

## Examples

### Default

<StaticUseLockBodyScroll />

## See Also

- [Dialog](/components/dialog)
- [Drawer](/components/drawer)
- [Sheet](/components/sheet)
