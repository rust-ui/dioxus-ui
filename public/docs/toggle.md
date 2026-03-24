+++
title = "Toggle"
description = "Rust/UI component that displays a two-state button that can be either on or off."
+++

<DemoToggle />

## Installation

Coming soon.

## Usage

```rust
use crate::ui::toggle::Toggle;
```

```rust
rsx! {
    Toggle {
        pressed: pressed(),
        onclick: move |_| pressed.set(!pressed()),
        "Bold"
    }
}
```

## See Also

- [Switch](/components/switch)
- [Toggle Group](/components/toggle-group)
