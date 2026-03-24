---
title: "Slider"
name: "slider"
cargo_dependencies: []
registry_dependencies: []
type: "components:ui"
path: "ui/slider.rs"
description: "Rust/UI component that allows users to select a value from a range."
tags: []
---

<DemoSlider />

## Installation

Coming soon.

## Usage

```rust
use crate::ui::slider::Slider;
```

```rust
let mut value = use_signal(|| 50.0_f64);

rsx! {
    Slider {
        value: value(),
        oninput: move |e: FormEvent| {
            if let Ok(v) = e.value().parse::<f64>() {
                value.set(v);
            }
        },
    }
}
```

## States

<DemoSliderDisabled />

## See Also

- [Progress](/components/progress)
- [Input](/components/input)
