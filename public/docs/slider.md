---
title: Slider
description: An input where the user selects a value from within a given range using a native range input.
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
