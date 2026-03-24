+++
title = "Select Native"
+++

## Installation

Coming soon.

## Usage

```rust
use crate::ui::select_native::SelectNative;
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

## Option Groups

<DemoSelectNativeGroup />

## See Also

- [Input](/components/input)
- [Label](/components/label)
