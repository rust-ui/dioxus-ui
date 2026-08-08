+++
title = "Marker"
description = "Rust/UI component for inline status markers, timeline separators, and event indicators."
+++

<DemoMarker />

## Installation

<InstallMarker />

## Usage

```rust
use crate::ui::marker::{Marker, MarkerContent, MarkerIcon};
```

```rust
rsx! {
    Marker {
        MarkerIcon { GitBranch {} }
        MarkerContent { "Switched to a new branch" }
    }
}
```

## Examples

### Variants

Use the `variant` prop to switch between `Default`, `Separator`, and `Border` layouts.

<DemoMarkerVariants />

### Border

<DemoMarkerBorder />

### Separator

Renders inline bars on either side of the content, useful for timeline dividers.

<DemoMarkerSeparator />

### Icon

<DemoMarkerIcon />

### Status

Pass `role="status"` for markers announcing in-progress work.

<DemoMarkerStatus />

### Shimmer

Combine `role="status"` with a `shimmer` class on `MarkerContent` for an animated loading state.

<DemoMarkerShimmer />

### Link / Button

Pass `href` to render an `<a>`, or `onclick` to render a `<button>`.

<DemoMarkerLinkButton />

## See Also

- [Badge](/components/badge)
- [Status](/components/status)
