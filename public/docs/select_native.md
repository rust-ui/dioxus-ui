---
title: Select Native
description: A native HTML select element with custom styling and a chevron icon.
tags: []
---

<DemoSelectNative />

## Installation

Add `dioxus_ui` to your `Cargo.toml`:

```toml
dioxus_ui = "0.1"
```

## Usage

```rust
use dioxus_ui::select_native::SelectNative;
```

```rust
let mut selected = use_signal(|| "apple".to_string());

rsx! {
    SelectNative {
        id: "fruit-select",
        onchange: move |e: FormEvent| selected.set(e.value()),
        option { value: "apple", "Apple" }
        option { value: "banana", "Banana" }
    }
}
```

## See Also

- [Input](/components/input)
- [Label](/components/label)
