---
title: "Table"
name: "table"
cargo_dependencies: ["tw_merge"]
registry_dependencies: []
type: "components:ui"
path: "ui/table.rs"
description: "Rust/UI component that displays a table with header, body and footer."
tags: []
---

# Table

Rust/UI component that displays a table with header, body and footer.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add table
```

## Component Code

```rust
use dioxus::prelude::*;
use tw_merge::tw_merge;

#[component]
pub fn TableWrapper(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!("overflow-auto rounded-md border max-h-96", class.as_deref().unwrap_or(""));
    rsx! { div { "data-name": "TableWrapper", class: "{merged}", {children} } }
}

#[component]
pub fn Table(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!("w-full text-sm caption-bottom", class.as_deref().unwrap_or(""));
    rsx! {
        div { class: "relative w-full overflow-x-auto rounded-xl border",
            table { class: "{merged}", {children} }
        }
    }
}

#[component]
pub fn TableHeader(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!("[&_tr]:border-b", class.as_deref().unwrap_or(""));
    rsx! { thead { class: "{merged}", {children} } }
}

#[component]
pub fn TableBody(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!("[&_tr:last-child]:border-0", class.as_deref().unwrap_or(""));
    rsx! { tbody { class: "{merged}", {children} } }
}

#[component]
pub fn TableRow(
    #[props(into, optional)] class: Option<String>,
    #[props(into, optional)] data_state: Option<String>,
    children: Element,
) -> Element {
    let merged = tw_merge!(
        "border-b transition-colors hover:bg-muted/50 data-[state=selected]:bg-muted",
        class.as_deref().unwrap_or("")
    );
    rsx! {
        tr {
            class: "{merged}",
            "data-state": data_state.as_deref(),
            {children}
        }
    }
}

#[component]
pub fn TableHead(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!(
        "h-10 px-4 text-left align-middle font-medium text-muted-foreground whitespace-nowrap",
        class.as_deref().unwrap_or("")
    );
    rsx! { th { class: "{merged}", {children} } }
}

#[component]
pub fn TableCell(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!("p-4 align-middle", class.as_deref().unwrap_or(""));
    rsx! { td { class: "{merged}", {children} } }
}

#[component]
pub fn TableFooter(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged =
        tw_merge!("font-medium border border-t bg-muted/50 [&>tr]:last:border-b-0", class.as_deref().unwrap_or(""));
    rsx! { tfoot { class: "{merged}", {children} } }
}

#[component]
pub fn TableCaption(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!("mt-4 text-sm text-muted-foreground", class.as_deref().unwrap_or(""));
    rsx! { caption { class: "{merged}", {children} } }
}
```
