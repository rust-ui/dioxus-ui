+++
title = "Callout"
description = "A callout component for displaying notes, info, and warnings."
+++

<DemoCallout />

## Installation

Coming soon.

## Usage

```rust
use crate::ui::callout::Callout;
```

```rust
rsx! {
    Callout { title: "Note", "Your message here." }
}
```

## Examples

### Info

<DemoCalloutInfo />

### Warning

<DemoCalloutWarning />

## See Also

- [Alert](/components/alert)
- [Badge](/components/badge)
