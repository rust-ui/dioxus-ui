+++
title = "Callout"
description = "A callout component for displaying notes, info, and warnings."
+++

<StaticCallout />

## Installation

<StaticInstallCallout />

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

<StaticCalloutInfo />

### Warning

<StaticCalloutWarning />

## See Also

- [Alert](/components/alert)
- [Badge](/components/badge)
