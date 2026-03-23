---
title: Slider
description: An input where the user selects a value from within a given range using a native range input.
tags: []
---

<DemoSlider />

## Installation

Add `dioxus_ui` to your `Cargo.toml`:

```toml
dioxus_ui = "0.1"
```

## Usage

```rust
use dioxus_ui::slider::Slider;
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

## Props

| Prop | Type | Default | Description |
|---|---|---|---|
| min | f64 | 0.0 | Minimum value |
| max | f64 | 100.0 | Maximum value |
| step | f64 | 1.0 | Step increment |
| value | f64 | 50.0 | Current value |
| disabled | bool | false | Disable interaction |

## See Also

- [Progress](/components/progress)
- [Input](/components/input)
