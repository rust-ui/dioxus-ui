---
title: "Use Data Scrolled"
name: "use_data_scrolled"
cargo_dependencies: ["wasm_bindgen"]
registry_dependencies: []
type: "components:hooks"
path: "hooks/use_data_scrolled.rs"
description: "This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality."
tags: []
---

# Use Data Scrolled

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add use_data_scrolled
```

## Component Code

```rust
use dioxus::prelude::*;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

pub const DATA_SCROLL_TARGET: &str = "data-scroll-target";

pub fn use_data_scrolled(threshold_px: u32) -> Signal<bool> {
    let is_data_scrolled_signal = use_signal(|| false);

    use_effect(move || {
        #[cfg(target_arch = "wasm32")]
        {
            let threshold = f64::from(threshold_px);
            let scroll_container =
                web_sys::window().and_then(|w| w.document()).and_then(|d| d.get_element_by_id(DATA_SCROLL_TARGET));

            let get_scroll_pos = {
                let container = scroll_container.clone();
                move || -> f64 {
                    if let Some(ref el) = container { el.scroll_top() as f64 } else { get_scroll_position() }
                }
            };

            *is_data_scrolled_signal.write_unchecked() = get_scroll_pos() > threshold;

            let closure = wasm_bindgen::closure::Closure::<dyn FnMut(web_sys::Event)>::new({
                let is_data_scrolled_signal = is_data_scrolled_signal;
                move |_: web_sys::Event| {
                    *is_data_scrolled_signal.write_unchecked() = get_scroll_pos() > threshold;
                }
            });

            if let Some(el) = scroll_container {
                let _ = el.add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref());
            } else if let Some(window) = web_sys::window() {
                let _ = window.add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref());
            }

            closure.forget();
        }
    });

    is_data_scrolled_signal
}

#[cfg(target_arch = "wasm32")]
fn sync_header_padding_with_body(padding: &str) {
    let _ = (|| -> Option<()> {
        let element =
            web_sys::window().and_then(|w| w.document())?.query_selector("[data-name='NavMenuFixed']").ok()??;
        let header_el = element.dyn_ref::<web_sys::HtmlElement>()?;

        if !padding.is_empty() && padding != "0px" {
            header_el.style().set_property("padding-right", padding).ok()?;
        } else {
            header_el.style().remove_property("padding-right").ok()?;
        }
        Some(())
    })();
}

#[cfg(target_arch = "wasm32")]
fn get_scroll_position() -> f64 {
    let Some(body) = web_sys::window().and_then(|w| w.document()).and_then(|d| d.body()) else {
        return web_sys::window().and_then(|w| w.scroll_y().ok()).unwrap_or(0.0);
    };

    let style = body.style();
    let is_fixed = style.get_property_value("position").ok() == Some("fixed".to_string());
    let padding = style.get_property_value("padding-right").unwrap_or_default();

    sync_header_padding_with_body(&padding);

    if is_fixed {
        style
            .get_property_value("top")
            .ok()
            .and_then(|top| top.strip_suffix("px")?.strip_prefix("-")?.parse().ok())
            .unwrap_or(0.0)
    } else {
        web_sys::window().and_then(|w| w.scroll_y().ok()).unwrap_or(0.0)
    }
}
```
