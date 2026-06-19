---
title: "Use Copy Clipboard"
name: "use_copy_clipboard"
cargo_dependencies: ["wasm_bindgen"]
registry_dependencies: []
type: "components:hooks"
path: "hooks/use_copy_clipboard.rs"
description: "A Dioxus hook that copies text to clipboard with optional timeout to show copied state."
tags: []
---

# Use Copy Clipboard

A Dioxus hook that copies text to clipboard with optional timeout to show copied state.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add use_copy_clipboard
```

## Component Code

```rust
use dioxus::prelude::*;

const DEFAULT_TIMEOUT_MS: i32 = 2000;

pub fn use_copy_clipboard(timeout_ms: Option<i32>) -> (impl Fn(&str) + Clone, ReadSignal<bool>) {
    let copied = use_signal(|| false);
    let _timeout = timeout_ms.unwrap_or(DEFAULT_TIMEOUT_MS);

    let copy_fn = move |_text: &str| {
        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::JsCast;

            let Some(window) = web_sys::window() else { return };
            let _ = window.navigator().clipboard().write_text(_text);

            *copied.write_unchecked() = true;

            let closure = wasm_bindgen::closure::Closure::once_into_js(move || {
                *copied.write_unchecked() = false;
            });
            let _ = window
                .set_timeout_with_callback_and_timeout_and_arguments_0(closure.as_ref().unchecked_ref(), _timeout);
        }
    };

    (copy_fn, copied.into())
}
```
